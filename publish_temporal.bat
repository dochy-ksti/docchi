cd ./docchi_intf
cargo publish
timeout /t 180 > nul
cd ../
cargo publish