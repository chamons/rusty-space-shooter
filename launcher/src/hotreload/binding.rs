use std::sync::{Arc, Mutex};

use anyhow::Result;

use example::game::types::{GameColor, Position, Size};
use wasmtime::component::{Component, Linker, Resource, ResourceAny};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

use exports::example::game::game_api::{GuestGameInstance, KeyboardInfo, MouseInfo};

use super::wasm_path;
pub use crate::GameScreen;

wasmtime::component::bindgen!({
    path: "../wit",
    with: {
        "example:game/host-api/game-screen": GameScreen,
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

impl example::game::host_api::Host for MyState {}
impl example::game::types::Host for MyState {}

impl example::game::host_api::HostGameScreen for MyState {
    fn draw_text(
        &mut self,
        screen: Resource<GameScreen>,
        text: String,
        position: Position,
        size: f32,
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

    fn drop(&mut self, screen: Resource<GameScreen>) -> wasmtime::Result<()> {
        debug_assert!(screen.owned());
        self.table.delete(screen)?;
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
    bindings: HotreloadExample,
    context: Arc<Mutex<WebAssemblyContext>>,
}

impl WebAssemblyInstance {
    pub fn load(mut context: WebAssemblyContext) -> Result<WebAssemblyInstance> {
        let wasm_path = wasm_path()?;

        let component = Component::from_file(&context.engine, wasm_path)?;

        let mut linker = Linker::new(&context.engine);
        HotreloadExample::add_to_linker(&mut linker, |state: &mut MyState| state)?;
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        let (bindings, _) = HotreloadExample::instantiate(&mut context.store, &component, &linker)?;
        Ok(Self {
            bindings,
            context: Arc::new(Mutex::new(context)),
        })
    }

    pub fn create_game_instance(&mut self) -> Result<GameInstance> {
        let instance_type = self.bindings.example_game_game_api().game_instance();

        let instance = {
            let mut context = self.context.lock().unwrap();
            instance_type.call_constructor(&mut context.store)?
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
    pub fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen) -> Result<()> {
        let mut context = self.context.lock().unwrap();
        let screen = context.store.data_mut().convert_to_resource(screen)?;

        self.instance_type
            .call_run_frame(&mut context.store, self.instance, mouse, &key, screen)
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
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen) {
        if let Err(e) = GameInstance::run_frame(self, mouse, key, screen) {
            println!("Error running frame: {e:?}");
        }
    }
}
