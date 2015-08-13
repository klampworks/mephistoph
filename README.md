# mephistoph
A sliding XOR implementation

This is just a small project to help me learn the Rust programming language.

# Usage
    Usage: ./mephistoph [options]

    Options:
        -k --key "my secret key"
                            A string to be used as a key. If shorter than data
                            will be cycled.
        --keyfile "~/my-key-file"
                            A file to be used as a key. If the contents is shorter
                            than the data it will be cycled.
                            
Input is taken from stdin and output written to stdout. 

# Examples

    echo hello | ./mephistoph -k 12345
    YW_XZ
    
Since it is XOR, applying the same key twice should give the original input.

    echo "hello" | ./mephistoph -k 12345 | ./mephistoph -k 12345
    hello
    
If the key is shorter than the input data then it is read in a circular fashion.

    echo "hello" | ./mephistoph -k 12
    YW]^^
    
    echo "hello" | ./mephistoph -k 12121
    YW]^^

Instead of specifiying the key on the command line, the contents of files can also be used as keys.

    echo "12345" | ./mephistoph -keyfile Cargo.toml 
    TKU]Y
    
Using shell IO redirection, the contents of files can be used as input too.

    ./mephistoph -k 12345 < Cargo.toml 
    jBRW^PUVi?_S^Q
                  YPAZZGA^B[?GWAG\^\        U@EZ\FFoHGX]EH]]        F]_RCSWRFuV_R]YQ\Y
     o9>nUWCQ[UW]W\TAn>RTF\DAB

And, of course the output can be directed to a file as well.

    ./mephistoph -k 12345 < Cargo.toml > Cargo.toml.xored

