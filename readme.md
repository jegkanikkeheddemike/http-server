# Cool hjemmelavet http-server
Den serverere html. Ikke meget andet. http-parseren er 100% zero-copy så i teorien er den super performant. Kinda bortset fra at den bruger en bufferen reader hvilket i guess kopiere en smule.

`cargo run --release` for at køre.