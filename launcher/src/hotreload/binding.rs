use std::sync::{Arc, Mutex};

use anyhow::Result;

use caffeinated_gorilla::space::host_api::Shader;
use caffeinated_gorilla::space::types::{GameColor, Position, Size};
use macroquad::prelude::*;
use wasmtime::component::{Component, Linker, Resource, ResourceAny};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use exports::caffeinated_gorilla::space::game_api::{GuestGameInstance, KeyboardInfo, MouseInfo};

use super::wasm_path;
pub use crate::GameScreen;

wasmtime::component::bindgen!({
    path: "../wit",
    with: {
        "caffeinated-gorilla:space/host-api/game-screen": GameScreen,
        "caffeinated-gorilla:space/host-api/shader": crate::shader::Shader,
    },
    trappable_imports: true,
});

pub struct MyState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl MyState {
    pub fn convert_to_resource<T>(&mut self, item: T) -> wasmtime::Result<Resource<T>>
    where
        T: Send + 'static,
    {
        let id = self.table.push(item)?;
        Ok(id)
    }
}

impl caffeinated_gorilla::space::host_api::Host for MyState {}

impl caffeinated_gorilla::space::types::Host for MyState {}

impl caffeinated_gorilla::space::host_api::HostGameScreen for MyState {
    fn draw_text(
        &mut self,
        screen: Resource<GameScreen>,
        text: String,
        position: Position,
        size: u16,
        color: GameColor,
    ) -> wasmtime::Result<()> {
        debug_assert!(!screen.owned());
        let screen = self.table.get(&screen)?;
        screen.draw_text(&text, position, size, color);
        Ok(())
    }

    fn draw_image(
        &mut self,
        screen: Resource<GameScreen>,
        filename: String,
        position: Position,
        size: Option<Size>,
    ) -> wasmtime::Result<()> {
        debug_assert!(!screen.owned());
        let screen = self.table.get(&screen)?;
        screen.draw_image(&filename, position, size);
        Ok(())
    }

    fn draw_line(
        &mut self,
        screen: Resource<GameScreen>,
        first: Position,
        second: Position,
        thickness: f32,
        color: GameColor,
    ) -> wasmtime::Result<()> {
        debug_assert!(!screen.owned());
        let screen = self.table.get(&screen)?;
        screen.draw_line(first, second, thickness, color);
        Ok(())
    }

    fn draw_circle(
        &mut self,
        screen: Resource<GameScreen>,
        position: Position,
        radius: f32,
        color: GameColor,
    ) -> wasmtime::Result<()> {
        debug_assert!(!screen.owned());
        let screen = self.table.get(&screen)?;
        screen.draw_circle(position, radius, color);
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        screen: Resource<GameScreen>,
        position: Position,
        size: Size,
        color: GameColor,
    ) -> wasmtime::Result<()> {
        debug_assert!(!screen.owned());
        let screen = self.table.get(&screen)?;
        screen.draw_rectangle(position, size, color);
        Ok(())
    }

    fn width(&mut self, screen: Resource<GameScreen>) -> wasmtime::Result<f32> {
        debug_assert!(!screen.owned());
        Ok(screen_width())
    }

    fn height(&mut self, screen: Resource<GameScreen>) -> wasmtime::Result<f32> {
        debug_assert!(!screen.owned());
        Ok(screen_height())
    }

    fn measure_text(
        &mut self,
        screen: Resource<GameScreen>,
        text: String,
        size: u16,
    ) -> wasmtime::Result<caffeinated_gorilla::space::host_api::TextDimensions> {
        debug_assert!(!screen.owned());
        let screen = self.table.get(&screen)?;
        let size = screen.measure_text(&text, size);
        Ok(caffeinated_gorilla::space::host_api::TextDimensions {
            width: size.width,
            height: size.height,
            offset_y: size.offset_y,
        })
    }

    #[doc = " This is hard coded to have an external"]
    #[doc = " UniformDesc::new(\"direction_modifier\", UniformType::Float1),"]
    #[doc = " If you need more, edit shaders directly"]
    fn load_shader(
        &mut self,
        screen: Resource<GameScreen>,
        fragment: String,
        vertex: String,
    ) -> wasmtime::Result<Resource<Shader>> {
        debug_assert!(!screen.owned());

        let shader = Shader::new(&fragment, &vertex)?;
        Ok(self.convert_to_resource(shader)?)
    }

    fn drop(&mut self, screen: Resource<GameScreen>) -> wasmtime::Result<()> {
        debug_assert!(screen.owned());
        self.table.delete(screen)?;
        Ok(())
    }
}

impl caffeinated_gorilla::space::host_api::HostShader for MyState {
    fn render(
        &mut self,
        shader: Resource<Shader>,
        direction_modifier: f32,
    ) -> wasmtime::Result<()> {
        debug_assert!(!shader.owned());
        let shader = self.table.get(&shader)?;
        shader.render(direction_modifier);
        Ok(())
    }

    fn drop(&mut self, shader: Resource<Shader>) -> wasmtime::Result<()> {
        debug_assert!(shader.owned());
        self.table.delete(shader)?;
        Ok(())
    }
}

pub struct WebAssemblyContext {
    store: Store<MyState>,
    engine: Engine,
}

impl WebAssemblyContext {
    pub fn load() -> Result<WebAssemblyContext> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;

        let mut wasi = WasiCtxBuilder::new();
        wasi.inherit_stdio();
        wasi.preopened_dir(".", ".", DirPerms::all(), FilePerms::all())?;

        let store = Store::new(
            &engine,
            MyState {
                ctx: wasi.build(),
                table: ResourceTable::new(),
            },
        );
        Ok(Self { store, engine })
    }
}

pub struct WebAssemblyInstance {
    bindings: SpaceShooterGame,
    context: Arc<Mutex<WebAssemblyContext>>,
}

impl WebAssemblyInstance {
    pub fn load(mut context: WebAssemblyContext) -> Result<WebAssemblyInstance> {
        let wasm_path = wasm_path()?;

        let component = Component::from_file(&context.engine, wasm_path)?;

        let mut linker = Linker::new(&context.engine);
        SpaceShooterGame::add_to_linker(&mut linker, |state: &mut MyState| state)?;
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        let (bindings, _) = SpaceShooterGame::instantiate(&mut context.store, &component, &linker)?;
        Ok(Self {
            bindings,
            context: Arc::new(Mutex::new(context)),
        })
    }

    pub fn create_game_instance(&mut self, screen: GameScreen) -> Result<GameInstance> {
        let instance_type = self
            .bindings
            .caffeinated_gorilla_space_game_api()
            .game_instance();

        let instance = {
            let mut context = self.context.lock().unwrap();
            let screen = context.store.data_mut().convert_to_resource(screen)?;
            instance_type.call_constructor(&mut context.store, screen)?
        };

        Ok(GameInstance {
            instance_type,
            instance,
            context: self.context.clone(),
        })
    }
}

pub struct GameInstance<'a> {
    instance_type: GuestGameInstance<'a>,
    instance: ResourceAny,
    context: Arc<Mutex<WebAssemblyContext>>,
}

impl GameInstance<'_> {
    pub fn update_frame(
        &self,
        mouse: MouseInfo,
        key: KeyboardInfo,
        screen: GameScreen,
    ) -> Result<()> {
        let mut context = self.context.lock().unwrap();
        let screen = context.store.data_mut().convert_to_resource(screen)?;

        self.instance_type.call_update_frame(
            &mut context.store,
            self.instance,
            mouse,
            &key,
            screen,
            get_frame_time(),
        )
    }

    pub fn render_frame(&self, screen: GameScreen) -> Result<()> {
        let mut context = self.context.lock().unwrap();
        let screen = context.store.data_mut().convert_to_resource(screen)?;

        self.instance_type
            .call_render_frame(&mut context.store, self.instance, screen)
    }

    pub fn save(&self) -> Result<Vec<u8>> {
        let mut context = self.context.lock().unwrap();

        self.instance_type
            .call_save(&mut context.store, self.instance)
    }

    pub fn load(&self, data: Vec<u8>) -> Result<()> {
        let mut context = self.context.lock().unwrap();

        self.instance_type
            .call_restore(&mut context.store, self.instance, &data)
    }
}

#[async_trait::async_trait]
impl crate::RunnableGameInstance for GameInstance<'_> {
    fn update_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen) {
        if let Err(e) = GameInstance::update_frame(self, mouse, key, screen) {
            println!("Error in updating frame: {e:?}");
        }
    }

    fn render_frame(&self, screen: GameScreen) {
        if let Err(e) = GameInstance::render_frame(self, screen) {
            println!("Error in rendering frame: {e:?}");
        }
    }
    fn save(&self) -> String {
        String::from_utf8(GameInstance::save(&self).unwrap_or_default()).unwrap_or_default()
    }
}
