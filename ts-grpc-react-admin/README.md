### Avored rust grpc cms

Generate as Typescript CLI

    protoc -I=../proto admin_user.proto  --js_out=import_style=commonjs,binary:src/grpc_generated --grpc-web_out=import_style=typescript,mode=grpcweb:src/grpc_generated

    protoc --proto_path=../proto/ admin_user.proto \
        --grpc-web_out=import_style=typescript,mode=grpcweb:src/grpc_generated \ 
        --js_out=import_style=commonjs,binary:src/grpc_generated


    protoc -I=../proto admin_user.proto \
        --js_out=import_style=commonjs:src/grpc_generated \
        --grpc-web_out=import_style=commonjs,mode=grpcwebtext:src/grpc_generated