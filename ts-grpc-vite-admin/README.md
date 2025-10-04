# Typescript AvoRed Rust gRPC admin app 


#### Dashboard PROTO generation command 


    protoc -I=./../proto  dashboard.proto \
        --js_out=import_style=commonjs,binary:src/grpc-avored \
        --grpc-web_out=import_style=typescript,mode=grpcweb:src/grpc-avored