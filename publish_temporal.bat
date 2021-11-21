cd ../docchi_intf
cargo publish
timeout /t 120 > nul
cd ../
cargo publish