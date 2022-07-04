FROM mcr.microsoft.com/dotnet/sdk:6.0

RUN mkdir -p /opt/dotnet
ENV DOTNET_CLI_HOME="/opt/dotnet"
RUN dotnet tool install -g dotnet-script

COPY run.sh /var/run
