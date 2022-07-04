FROM mcr.microsoft.com/dotnet/sdk:6.0

ENV DOTNET_CLI_HOME="/opt"
RUN dotnet tool install dotnet-script --tool-path /opt/.dotnet/tools
ENV PATH="${PATH}:${DOTNET_CLI_HOME}/.dotnet/tools" 	

COPY run.sh /var/run
