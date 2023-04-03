# machine_learning

Welcome to your new machine_learning project and to the Internet Computer's Development Community. This is demonstrates machine learning running on Dfinity's Internet Computer Protocol - a decentralized platform that can displace the use of centrally-owned infrastructure.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with machine_learning, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd machine_leanring/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
