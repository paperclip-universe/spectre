#!/usr/bin/env bash
echo "Script executed from: ${PWD}"

needCommand() {
    if ! command -v "$1" &>/dev/null; then
        echo "=== === Command needed! === ==="
        echo "You need to install $1 and have it avalibale in your \$PATH!"
        exit
    fi
}

needCommand java
needCommand megadl
needCommand unzip
needCommand python2
needCommand scalac

echo ">>> Downloading required assets"
megadl "https://mega.nz/file/d91BCbBZ#RsqXaRyIIhL8rHuIo8Xd-DfTxpM4X_CBvUav8RjuESI"
wget "https://launcher.mojang.com/v1/objects/b58b2ceb36e01bcd8dbf49c8fb66c55a9f0676cd/server.jar" -O "server.jar"

echo ">>> Unzipping assets"
unzip MCP_Tutorial.zip -d tmp_mcp
echo ">> Cleanup"
rm -fv MCP_Tutorial.zip

echo ">>> Bootstrapping MCP"
echo ">> Extracting"
mv -v ./tmp_mcp/files/mcp918.zip ./
unzip mcp918.zip -d minecraft
echo ">> Cleanup"
rm -fv mcp918.zip

echo ">>> Moving server jar"
mv -v ./server.jar ./minecraft/jars/minecraft_server.1.8.9.jar

echo ">>> Setting up SRG mappings"
mv -v ./tmp_mcp/files/mcp-1.8.9-srg.zip ./
echo ">> Extracting"
unzip mcp-1.8.9-srg.zip -d tmp_srg
echo ">> Cleanup"
rm -fv mcp-1.8.9-srg.zip

echo ">>> Setting up nodoc mappings"
mv -v ./tmp_mcp/files/mcp_stable_nodoc-22-1.8.9.zip ./
echo ">> Extracting"
unzip mcp_stable_nodoc-22-1.8.9.zip -d tmp_nodoc
echo ">> Cleanup"
rm -fv mcp_stable_nodoc-22-1.8.9.zip
rm -rvf tmp_mcp

echo ">>> Merging SRG & nodoc"
mv -v tmp_nodoc/* tmp_srg/
echo ">> Cleanup"
rm -rvf tmp_nodoc

echo ">>> Moving merged mappings"
cd tmp_srg || exit
find . -type d -exec mkdir -vp "../minecraft/conf"/{} \; -or -exec mv -v {} "../minecraft/conf"/{} \;
cd ..
echo ">> Cleanup"
rm -rvf tmp_srg

echo ">>> Setting version to 1.8.9"
cat <<EOF >./minecraft/conf/version.cfg
[VERSION]
MCPVersion = 9.18
ClientVersion = 1.8.9
ServerVersion = 1.8.9
EOF

echo ">>> Passing execution to MCP"
cd minecraft || exit
python2 runtime/decompile.py --norecompile

echo ">>> Deleting rejects"
find . | rg ".(rej|orig)" | xargs rm
