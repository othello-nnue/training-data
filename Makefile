FILE_NAME := knowledge_archive.tar.bz2
OUTPUT_FILE := knowledge_archive.tar.zst
DOWNLOAD_URL := https://figshare.com/ndownloader/files/42860026
EXPECTED_BLAKE3 := ab37f79c537df1c22479cf5a7a72f76bb005cabae8f09cb23e5301227a42929e
UNZIPPED_BLAKE3 := 7467a1979b944b168635b47854913b19fa637c7f9ecf849d5052dbc83c759d5f

DEPENDENCIES := aria2c b3sum bzcat zstd

# Check if dependencies are installed
check_dependencies:
	@$(foreach dep,$(DEPENDENCIES), which $(dep) >/dev/null || (echo "$(dep) is not installed." && exit 1);)

all: check_dependencies $(OUTPUT_FILE)

$(FILE_NAME):
	@echo "Downloading $(FILE_NAME)..."
	@aria2c -c "$(DOWNLOAD_URL)"
	@echo "Downloaded $(FILE_NAME) successfully."
	@b3sum -c <<< "$(EXPECTED_BLAKE3)  $(FILE_NAME)" || (echo "File verification failed." && exit 1)
	@echo "File integrity verified."

$(OUTPUT_FILE): $(FILE_NAME)
	@echo "$(OUTPUT_FILE) not found. Recompressing from $(FILE_NAME)..."
	@bzcat "$<" | zstd -19 -T0 -o "$@"
	@echo "Conversion completed: $@"

output.zst: $(OUTPUT_FILE)
	@cargo run --release | zstd -19 -T0 -o output.zst

verify: $(OUTPUT_FILE)
	@echo "Verifying unzipped content..."
	@actual_blake3=$$(zstdcat "$(OUTPUT_FILE)" | b3sum -); \
	if [ "$$actual_blake3" == "$(UNZIPPED_BLAKE3)  -" ]; then \
		echo "Unzipped file integrity verified."; \
	else \
		echo "Unzipped file verification failed."; \
		echo "Expected: $(UNZIPPED_BLAKE3)"; \
		echo "Actual: $$actual_blake3"; \
		exit 1; \
	fi

clean:
	@rm -f $(FILE_NAME) $(OUTPUT_FILE)

.PHONY: all check_dependencies verify clean
