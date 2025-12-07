#!/bin/bash
# 使用 GNU parallel 并行处理（如果安装了 parallel）
find C/github -name "*.c" -type f | parallel /usr/bin/python /work/CRTrans/transpile.py --c-file {} --api-key sk-64eefce97b664c8d8d45ed76a012a738