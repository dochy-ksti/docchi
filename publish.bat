cd ./docchi_json5
cargo publish
timeout /t 30 > nul
cd ../docchi_compaction
cargo publish
timeout /t 30 > nul
cd ../docchi_archiver2
cargo publish
timeout /t 30 > nul
cd ../docchi_core
cargo publish
timeout /t 30 > nul
cd ../docchi_diff
cargo publish
timeout /t 30 > nul
cd ../docchi_fs
cargo publish
timeout /t 30 > nul
cd ../docchi_intf
cargo publish
timeout /t 30 > nul
cd ../
cargo publish