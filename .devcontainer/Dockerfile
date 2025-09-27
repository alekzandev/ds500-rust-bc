FROM mcr.microsoft.com/devcontainers/rust:dev-1-bullseye

RUN apt update && export DEBIAN_FRONTEND=noninteractive \
    && apt install zsh vim -y

RUN curl -1sLf 'https://dl.cloudsmith.io/public/task/task/setup.deb.sh'\
    | sudo bash -E && apt install task