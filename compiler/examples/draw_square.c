int main() {
	int key = 0;
	unsigned int width = 384 * 2;
	unsigned int vram_start = 2885681152;

	__syscall(3755, &key, 0, 0, 0);

	while (0 == 0) {
		__syscall(626, 0, 0, 0, 0);

		*(vram_start + 1 * width + 3 * 2) = 0;

		for (int y = 0; y < 10; y = y + 1) {
			for (int x = 0; x < 10; x = x + 1) {
				*(vram_start + y * width + x * 2) = 0;
			}
		}
		
		__syscall(607, 0, 0, 0, 0);

		__syscall(3755, &key, 0, 0, 0);
	}

	return 0;
}