use std::str::FromStr;

use godot::classes::file_access::ModeFlags;
use godot::prelude::*;

mod godot_tokio;

use firesidexr_evergreen::types::*;
use libp2p::{identity::Keypair, Multiaddr};
use firesidexr_evergreen::network::untrusted::Network;

use godot::builtin::{Transform3D, Vector3};

#[derive(GodotClass)]
#[class(base=Resource)]
struct AvatarData {

    #[export] display_name: GString,
    #[export] head: GString,
    #[export] torso: GString,
    #[export] backpack: GString,
    #[export] primary_color: GString,
    #[export] accent_color: GString,

    base: Base<Resource>
}


#[derive(GodotClass)]
#[class(base=RefCounted)]
struct IdentityKeypair {
    keypair: Keypair
}

#[godot_api]
impl IRefCounted for IdentityKeypair {
    fn init(_base: Base < RefCounted >) -> Self {
        Self {
            keypair: Keypair::generate_ed25519()
        }
    }

    fn to_string(&self) -> godot::builtin::GString {
        GString::from(&self.keypair.public().to_peer_id().to_string())
    }
}


#[godot_api]
impl IdentityKeypair {

    #[func]
    fn save_to_file(&self, path: GString) {
        
        let mut file = FileAccess::open(&path, ModeFlags::WRITE).unwrap();

        file.store_buffer(&PackedByteArray::from(self.keypair.to_protobuf_encoding().expect("Failed to create protobuff")));
    }

    #[func]
    fn load_from_file(path: GString) -> Gd<Self> {

        let file = FileAccess::open(&path, ModeFlags::READ).unwrap();

        let buffer = file.get_buffer(file.get_length() as i64);

        let keypair = Keypair::from_protobuf_encoding(buffer.as_slice()).expect("Decoding error");

        Gd::from_init_fn(|_| {
            Self { keypair }
        })
    }


}



#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct Identity {

    jwt: String,

    avatar: Gd<AvatarData>
}

// impl Into<Avatar> for AvatarData {
//     fn into(self) -> Avatar {
//         Avatar { 
//             display_name: self.display_name.into(), 
//             head: self.head.into(), 
//             torso: self.torso.into(), 
//             backpack: self.backpack.into(), 
//             primary_color: self.primary_color.into(), 
//             accent_color: self.accent_color.into() 
//         }
//     }
// }

#[godot_api]
impl IResource for AvatarData {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            display_name: "".into(),
            head: "".into(), 
            torso: "".into(), 
            backpack: "".into(), 
            primary_color: "".into(), 
            accent_color: "".into() 
        }
    }
}


#[derive(GodotClass)]
#[class(base=Node, no_init)]
struct EvergreenNetworker {
    base: Base<Node>,
    connection: Option<NetworkHandle>,
    join_handle: Option<tokio::task::JoinHandle<()>>,

    ports_open: bool,

}



#[godot_api]
impl EvergreenNetworker {

    #[func]
    fn new_untrusted_networker(address: String, keypair: Option<Gd<IdentityKeypair>>) -> Gd<Self> {
        todo!()
    }

    #[func]
    fn new_trusted_networker(address: String) -> Gd<Self> {
        todo!()
    }

    #[func]
    fn new_p2p_networker(address: String) ->Gd<Self> {
        todo!()
    }






    /// Emitted when connected to a server
    #[signal]
    fn connected();

    /// Emitted when disconnected from a server
    #[signal]
    fn disconnected();


    

    /// Emitted when a message is recieved from another peer
    #[signal]
    fn message( peer: StringName, text: GString );

    /// Emitted when a peer sends a movement update
    #[signal]
    fn movement( peer: StringName, head: Transform3D, right_hand: Transform3D, left_hand: Transform3D );

    /// Emitted when a peer updates their avatar data
    #[signal]
    fn set_avatar( peer: StringName, avatar: Gd<AvatarData>);

    /// Emitted when another client updates the position of a "physics" object
    #[signal]
    fn puppet( id: i64, transform: Transform3D );

}





/// This class represents a connection to an untrusted server
/// 
/// Important to note: All signals emitted about a peer represent a signed message from that specfic peer.
/// You can trust this information directly.
#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
struct EvergreenUntrusted {

    client_event_handle: NetworkHandle,

    task_handle: tokio::task::JoinHandle<()>,

    //network: firesidexr_evergreen::network::untrusted::Network,

    base: Base<RefCounted>,
}


#[godot_api]
impl EvergreenUntrusted {

    /// Emitted when connected to a server
    #[signal]
    fn connected();

    /// Emitted when disconnected from a server
    #[signal]
    fn disconnected();


    #[signal]
    fn voip_data( peer: StringName, data: PackedByteArray );

    /// Emitted when a message is recieved from another peer
    #[signal]
    fn message( peer: StringName, text: GString );

    /// Emitted when a peer sends a movement update
    #[signal]
    fn movement( peer: StringName, head: Transform3D, right_hand: Transform3D, left_hand: Transform3D );

    /// Emitted when a peer updates their avatar data
    #[signal]
    fn set_avatar( peer: StringName, avatar: Gd<AvatarData>);

    /// Emitted when another client updates the position of a physics object
    #[signal]
    fn puppet( id: i64, transform: Transform3D );




    ///Creates a new client network instance that connects to the addr with a random identity
    #[func]
    fn new_connection_with_random_identity(address: String) -> Gd<Self> {

        let addr = Multiaddr::from_str(&address).expect("Invalid address");

        let keypair = Keypair::generate_ed25519();

        

        let (mut network, network_handle) = AsyncRuntime::block_on(
            async move {Network::new_client(keypair, addr)
                .expect("Could not create client struct")}
        );


        //let x = AsyncRuntime::spawn(async {godot_print!("Hello from another task!")});

        //godot_print!("{:?}", AsyncRuntime::block_on(async {x.await}));

        let async_handle = AsyncRuntime::spawn(async move { network.run().await; });

        Gd::from_init_fn(|base| {
            Self {

                //network,

                //test_handle,

                client_event_handle: network_handle,

                task_handle: async_handle,
    
                base,
            }
        })
        
    }

    /// Send avatar data to all connected peers
    #[func]
    fn send_avatar(&self, avatar: Gd<AvatarData>) {

        let av_ref = avatar.bind();

        // TODO: Find a better way to do this
        let avatar_rust = Avatar {
            display_name: av_ref.display_name.clone().into(),
            head: av_ref.head.clone().into(),
            torso: av_ref.torso.clone().into(),
            backpack: av_ref.backpack.clone().into(),
            primary_color: av_ref.primary_color.clone().into(),
            accent_color: av_ref.accent_color.clone().into()
        };

        self.client_event_handle.send_command_blocking(Command::SendData(PacketData::SetAvatar(avatar_rust)));
    }

    /// Sends a piece of text to all clients. Used for debugging network connections.
    #[func]
    fn send_text(&self, text: String) {

        self.client_event_handle.send_command_blocking(Command::SendData(PacketData::Message(text)));

    }

    /// Used to recieve updates about the state of the network and peers
    /// 
    /// Returns true while there are incoming events, and false otherwise
    /// Usage is suggested as 
    /// [code]
    /// while network.poll_event():
    ///     pass
    /// [/code]
    #[func]
    fn poll_event(&mut self) -> bool {
        let Some(event) =  self.client_event_handle.get_event_blocking() else {return false};

        godot_print!("{:?}", &event);

        

        match event {
            Response::Server(_server_response) => {
                
                
            },
            Response::Client(client_response) => { match client_response.data {

                PacketData::Voice(data) => self.signals().voip_data().emit(&client_response.peer.to_base58(), &PackedByteArray::from(data)),

                PacketData::Message(text) => self.signals().message().emit(&client_response.peer.to_base58(), &text),

                PacketData::Movement(floats) => {

                    let vectors: Vec<Vector3> = floats.iter().map(|f| { Vector3{x: f[0], y: f[1], z: f[2]} }).collect();

                    let head = Transform3D::from_cols(vectors[0], vectors[1], vectors[2], vectors[3]);

                    let right_hand = Transform3D::from_cols(vectors[4], vectors[5], vectors[6], vectors[7]);

                    let left_hand = Transform3D::from_cols(vectors[8], vectors[9], vectors[10], vectors[11]);

                    self.signals().movement().emit(&client_response.peer.to_base58(), head, right_hand, left_hand);

                },

                // PacketData::Puppet(id,floats) => {

                //     let vectors: Vec<Vector3> = floats.iter().map(|f| { Vector3{x: f[0], y: f[1], z: f[2]} }).collect();

                //     self.signals().puppet().emit(id, Transform3D::from_cols(vectors[0], vectors[1], vectors[2], vectors[3]));
                // },
                PacketData::SetAvatar(avatar) => {

                    let data = Gd::from_init_fn(|base| {
                        AvatarData {
                            display_name: (&avatar.display_name).into(),
                            head: (&avatar.head).into(),
                            torso: (&avatar.torso).into(),
                            backpack: (&avatar.backpack).into(),
                            primary_color: (&avatar.primary_color).into(),
                            accent_color: (&avatar.accent_color).into(),
                            base
                        }
                    });

                    self.signals().set_avatar().emit(&client_response.peer.to_base58(), &data);
                },
                // TODO: Add corresponding signals and such
                PacketData::AddPassport(passport)=> todo!(),
                //PacketData::GiveItem(id, peer) => todo!(),
            };},
            Response::Network(network_update) => {match network_update {
                NetworkUpdate::AliveWithAddr(addr) => godot_print!("Evergreen Untrusted is alive with addr: {addr}"),
                NetworkUpdate::NewPeer(_peer_id) => self.signals().connected().emit(),
                NetworkUpdate::PeerDisconnected(_peer_id) => self.signals().disconnected().emit(),
                NetworkUpdate::Disconnected => (),
                };},
        };

        return true
    }

    /// Whether this network instance is still valid
    #[func]
    fn is_invalid(&self) -> bool {
        //godot_print!("client: {}", self.client_event_handle.is_closed());
        //godot_print!("task: {}", self.task_handle.is_finished());
        self.client_event_handle.is_closed() || self.task_handle.is_finished()
    }

}



use godot::classes::{Engine, FileAccess};
use godot_tokio::AsyncRuntime;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {

     fn on_stage_init(level: InitStage) {
        match level {
            InitStage::Scene => {
                let mut engine = Engine::singleton();

                // This is where we register our async runtime singleton.
                engine.register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
            }
            _ => (),
        }
    }

    fn on_stage_deinit(level: InitStage) {
        match level {
            InitStage::Scene => {
                let mut engine = Engine::singleton();

                // Here is where we free our async runtime singleton from memory.
                if let Some(async_singleton) = engine.get_singleton(AsyncRuntime::SINGLETON) {
                    engine.unregister_singleton(AsyncRuntime::SINGLETON);
                    async_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        AsyncRuntime::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}

