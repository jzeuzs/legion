FROM mcr.microsoft.com/dotnet/sdk:6.0

RUN dotnet tool install -g dotnet-script
ENV PATH=/.dotnet/tools:${PATH}
COPY run.sh /var/run
