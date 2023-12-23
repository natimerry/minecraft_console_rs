# TO USE:
- create input_fifo with `mkfifo input_fifo`
- place password_list.txt and binary in the same folder (or use cargo run)
- `while [ 1 ] ; do cat <directory-of-binary>/input_fifo ; done | <start command> > <directory-of-binary>/server_output`
- run the binary