use ipc_init::Init;

fn main() {
    Init::new("/tmp/init/init.sock").reboot();
}
