#! /bin/bash

color_blue="\x1B[1;94m"
color_reset="\x1B[0m"

if [ "$#" -ne 1 ]; then
    echo "$0" >&2
    echo "  run the verifier on a single file with logging of air, vir, and smt to files in 'log/' in the same dire as the file" >&2
    echo "usage: $0 <file>" >&2
    exit -1
fi

rs_file=$1
rs_file_dir=`dirname "$rs_file"`
rs_file_basename=`basename "$rs_file"`

mkdir -p $rs_file_dir/log

./tools/rust-verify.sh $rs_file \
        --log-air $rs_file_dir/log/$rs_file_basename.air --log-vir $rs_file_dir/log/$rs_file_basename.vir --log-vir-simple $rs_file_dir/log/$rs_file_bbasename.vir-simple --log-smt $rs_file_dir/log/$rs_file_basename.smt
result=$?

echo
echo -e "${color_blue}log-air${color_reset}" "$rs_file_dir/log/$rs_file_basename.air"
echo -e "${color_blue}log-vir${color_reset}" "$rs_file_dir/log/$rs_file_basename.vir"
echo -e "${color_blue}log-vir-simple${color_reset}" "$rs_file_dir/log/$rs_file_basename.vir-simple..."
echo -e "${color_blue}log-smt${color_reset}" "$rs_file_dir/log/$rs_file_basename.smt"
exit $?
