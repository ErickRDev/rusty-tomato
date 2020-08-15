target_dir='/mnt/d/Stuff/Learning/Rust/rusty-tomato/'

echo 'Bundling files..'
zip -r bundled.zip * -x 'target/*'
echo -e '\t..done'

if [ ! -d $target_dir ]
then
    echo 'Target dir does not exist; creating it..'
    mkdir $target_dir
    echo -e '\t..done'
fi

echo 'Copying to host OS: '$target_dir
cp -f 'bundled.zip' $target_dir
echo -e '\t..done'

target_file=$target_dir'bundled.zip'

cd $target_dir

echo 'Unbundling file: '$target_file
unzip -u -o $target_file
echo -e '\t..done'

echo 'Building application in host OS..'
cmd.exe /c 'cargo run'