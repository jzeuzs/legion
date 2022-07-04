FROM mcr.microsoft.com/dotnet/sdk:6.0

RUN mkdir -p /opt/dotnet
ENV DOTNET_CLI_HOME="/opt/dotnet" \
    PATH="${DOTNET_CLI_HOME}/.dotnet/tools:${PATH}"

RUN dotnet tool install dotnet-script --tool-path $DOTNET_CLI_HOME/.dotnet/tools	

COPY run.sh /var/run
