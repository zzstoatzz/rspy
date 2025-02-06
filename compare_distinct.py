from random import randint
from time import perf_counter

from prefect.utilities.collections import distinct as py_distinct
from rich.console import Console
from rspy_utilities import distinct as rust_distinct

console = Console()


def benchmark(name: str, size: int) -> None:
    data = [randint(0, size // 2) for _ in range(size)]

    # Warmup
    list(py_distinct(data))
    list(rust_distinct(data))

    # Benchmark
    py_start = perf_counter()
    list(py_distinct(data))
    py_time = perf_counter() - py_start

    rust_start = perf_counter()
    list(rust_distinct(data))
    rust_time = perf_counter() - rust_start

    speedup = py_time / rust_time
    console.print(f"\nDistinct - Size: {size:,}")
    console.print(f"Python: {py_time:.4f}s")
    console.print(f"Rust:   {rust_time:.4f}s")
    console.print(f"Speedup: {speedup:.2f}x")


if __name__ == "__main__":
    for size in [100_000, 1_000_000]:
        benchmark("distinct", size)
