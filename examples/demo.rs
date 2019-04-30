fn main() {
    use std::net::TcpStream;
    use trovochat::commands::PrivMsg;
    use trovochat::{Client, UserConfig, Writer};

    // create a userconfig
    let userconfig = UserConfig::builder()
        .nick(std::env::var("MY_TROVO_NAME").unwrap())
        .token(std::env::var("MY_TROVO_PASS").unwrap())
        // enable these capbilities
        .tags()
        .membership()
        .commands()
        // build the config
        .build() // enable these tags
        .expect("semi-valid config");

    // connect to trovo
    let read = TcpStream::connect(trovochat::TROVO_IRC_ADDRESS).expect("connect");
    // clone the tcpstream
    let write = read.try_clone().expect("must be able to clone");

    // create the read adapter from the TcpStream
    let read = trovochat::SyncReadAdapter::new(read);

    // create a new client from the read, write pairs
    let mut client = Client::new(read, write);

    // when we receive a PrivMsg run this function
    // tok allows us to remove this later, if we want
    let _tok = client.on(move |msg: PrivMsg, w: Writer<_>| {
        const KAPPA: usize = 25;
        // print out `user: message`
        println!("{}: {}", msg.display_name().unwrap(), msg.message());

        let kappas = msg
            .emotes()
            .iter()
            // filter Kappas
            .filter(|e| e.id == KAPPA)
            // count how many times it appears
            .map(|d| d.ranges.len())
            .sum::<usize>();

        // if someone sent more than 3 Kappas, send a Kappa back
        if kappas >= 3 {
            // using the provided Writer
            w.send(msg.channel(), "Kappa").unwrap();
        }
    });

    // log if the broadcaster, a sub or a mod talks
    client.on(move |msg: PrivMsg, _: Writer<_>| {
        use trovochat::BadgeKind::{Broadcaster, Subscriber};

        let name = msg.display_name().unwrap_or_else(|| msg.user());
        let badges = msg
            .badges()
            .iter()
            // filter to just the "BadgeKind"
            .map(|badge| badge.kind.clone())
            .collect::<Vec<_>>();

        match (
            badges.contains(&Broadcaster),
            badges.contains(&Subscriber),
            msg.moderator(), // or badges.contains(&Moderator)
        ) {
            (true, _, _) => println!("{} is the broadcaster", name),
            (_, true, _) => println!("{} is a subscriber", name),
            (_, _, true) => println!("{} is a mod", name),
            (_, _, _) => {
                // just a normal viewer
            }
        };
    });

    // 'register' (sends out creds.) with the server
    client.register(userconfig).expect("register with trovo");

    // blocks the thread until the server tells us who we were
    match client.wait_for_ready() {
        // and print it out
        Ok(user) => {
            // id: 23196011, name: Some("museun"), color: Some(OrangeRed)
            println!(
                "id: {}, name: {:?}, color: {:?}",
                user.user_id, user.display_name, user.color
            )
        }
        Err(trovochat::ReadError::Inner(trovochat::Error::InvalidRegistration)) => {
            eprintln!("invalid nick/pass");
            std::process::exit(1);
        }
        Err(err) => panic!(err),
    };

    // get a clone of the writer, this allows you to write to the connection
    let w = client.writer();
    // join a channel
    w.join("museun").unwrap();

    {
        // not needed here, but the writer is clonable
        // you can also get another one from the `client`
        let w = w.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(3));
            w.send("museun", "VoHiYo").unwrap();
        });
    }

    // block this thread until the connection ends
    // this will call the filters when it receives the appropirate message
    if let Err(err) = client.run() {
        eprintln!("error while running: {}", err);
        std::process::exit(1);
    }
}
