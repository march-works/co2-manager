## copilot上での動作確認
1. (初回の場合)session managerのインストール
    ```
    curl "https://s3.amazonaws.com/session-manager-downloads/plugin/latest/ubuntu_64bit/session-manager-plugin.deb" -o "session-manager-plugin.deb"
    sudo dpkg -i session-manager-plugin.deb
    ```
1. copilotのコマンドでアプリケーションのshellに入る
    ```shell
    copilot svc exec -a co2-manager -e test
    ```
1. grpcurlのインストール
    ```
    curl -sSL "https://github.com/fullstorydev/grpcurl/releases/download/v1.8.7/grpcurl_1.8.7_linux_x86_64.tar.gz" | tar -xz -C /usr/local/bin
    ```
1. gRPC methodの実行
    ```
    grpcurl -plaintext -d '{"id": "1234"}' localhost:50051 user.User/getUser
    ```
