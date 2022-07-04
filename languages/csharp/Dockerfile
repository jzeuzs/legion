FROM mcr.microsoft.com/dotnet/sdk:6.0

RUN dotnet tool install dotnet-script --tool-path /opt/.dotnet/tools && \
    chmod +x /opt/.dotnet

COPY run.sh /var/run
