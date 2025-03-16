### Avored rust grpc cms

Generate as Typescript CLI

    protoc --proto_path=../proto/ auth.proto \
    --grpc-web_out=import_style=typescript,mode=grpcweb:src/grpc_generated \ 
    --js_out=import_style=commonjs,binary:src/grpc_generated