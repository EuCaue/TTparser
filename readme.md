# TTparser

TTparser is a minimalistic command-line tool written in Rust for converting terminal themes between various emulators and formats. Currently, it supports converting Kitty themes to Foot and Alacritty.

## Usage

1. Clone the TTparser repository:

   ```bash
   git clone https://github.com/EuCaue/TTparser.git
   ```

2. Navigate to the project directory:

   ```bash
   cd TTparser
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

4. Run TTparser to convert a Kitty theme to Foot and Alacritty:

   ```bash
   ./target/release/ttparser
   ```

5. Run TTparser to convert a Kitty theme to Alacritty only:

   ```bash
   ./target/release/ttparser --terminal-output=alacritty
   ```

5. Help command:  

   ```bash
   ./target/release/ttparser --help 
   ```

## Contribution

Contributions are welcome! _(since I'm learning rust, probably has a lot of things to improve)_ Fork this repository, make your changes, and submit a pull request.

## License

TTparser is licensed under the GPL3 License.

Thank you for using TTparser for your terminal theme customization needs!
