.PHONY: all build test run clean help

# 默认目标：重新构建并执行测试
all: clean build test

# 编译项目
build:
	cargo build
	cargo build --release

# 运行标准测试和文档测试
test:
	cargo nextest run
	cargo test --doc

# 运行项目
run:
	cargo run --release -q -p the-next-week-app -- 3 > images/TheNextWeek/output.ppm

# 清理项目
clean:
	cargo clean

# 仅构建
b: build

# 仅构建和测试
bt: build test

# 显示帮助信息
help:
	@echo "可用命令:"
	@echo "  all      - 重新编译并执行测试"
	@echo "  build    - 编译（debug + release）"
	@echo "  test     - 运行测试（nextest + test）"
	@echo "  run      - 运行项目（默认生成 images/output.ppm）"
	@echo "  clean    - 清理"
	@echo "  bt       - 构建并测试"
	@echo "  help     - 显示帮助信息"
	@echo ""
	@echo "使用示例: make all 或 make bt"
