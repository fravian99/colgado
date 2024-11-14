rm ./target/final -rdf
mkdir ./target/final
mkdir ./target/final/linux
mkdir ./target/final/windows
cp ./target/release/colgado ./target/final/linux
cp ./target/x86_64-pc-windows-gnu/release/colgado.exe ./target/final/windows
cp env.toml ./target/final/linux
cp env.toml ./target/final/windows
cd target/final;zip -rj colgado-windows windows
cd target/final;zip -rj colgado-linux linux
