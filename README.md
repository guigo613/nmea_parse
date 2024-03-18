# Cloning the Repository with Submodules

Make sure Git is installed on your system before proceeding.

To clone this repository and its submodules, execute the following command:

```shell
git clone --recurse-submodules https://github.com/guigo613/nmea_parse.git
```

If you have already cloned the repository without the submodules, you can update them using:

```shell
git submodule update --init --recursive
```

## Testing

To test an application built in Rust using Cargo, follow these steps:

1. Navigate to the root directory of your Rust application.
2. Ensure that the necessary dependencies are listed in your Cargo.toml file.
3. Run the following command to build and test your Rust application:
   
   ```shell
   cargo test
   ```

4. Review the test output to ensure that all tests pass successfully.

Optionally, you can execute a script located in the test directory called "parse_stdin_test.sh", passing the compiled project executable as an argument. This script may serve additional testing purposes.

```shell
./test/parse_stdin_test.sh path/to/your/executable
```

This script uses "parse_stdin_test_in.txt" as input and generates "parse_stdin_test_out.txt". It then compares the output with "parse_stdin_test_out_expected.txt". If there are any differences, it will alert you via the console.