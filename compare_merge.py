from random import choice, randint
from string import ascii_letters
from time import perf_counter

from rich.console import Console
from rspy_utilities import deep_merge_dicts as rust_merge

console = Console()


def py_merge(*dicts):
    result = {}
    for d in dicts:
        for k, v in d.items():
            if k in result and isinstance(result[k], dict) and isinstance(v, dict):
                result[k] = py_merge(result[k], v)
            else:
                result[k] = v
    return result


def random_string(length: int = 10) -> str:
    return "".join(choice(ascii_letters) for _ in range(length))


def generate_dict(depth: int, width: int, value_size: int = 100) -> dict:
    """Generate a realistic nested dictionary.

    Args:
        depth: Maximum nesting depth
        width: Number of keys at each level
        value_size: Size of string values for leaf nodes
    """
    if depth <= 0:
        return {
            "id": randint(0, 1000000),
            "name": random_string(value_size),
            "data": {str(i): random_string(value_size) for i in range(5)},
        }

    return {
        random_string(10): generate_dict(depth - 1, width, value_size)
        for _ in range(width)
    }


def benchmark(depth: int, width: int, n_dicts: int = 3) -> None:
    """Run benchmark with specified parameters."""
    dicts = [generate_dict(depth, width) for _ in range(n_dicts)]
    total_keys = sum(len(str(d)) for d in dicts)  # Rough size estimate

    console.print(f"\nMerge - Depth: {depth}, Width: {width}")
    console.print(f"Total data size: ~{total_keys/1024:.1f}KB")

    # Warmup
    py_merge(*dicts)
    rust_merge(*dicts)

    # Benchmark
    py_start = perf_counter()
    py_result = py_merge(*dicts)
    py_time = perf_counter() - py_start

    rust_start = perf_counter()
    rust_result = rust_merge(*dicts)
    rust_time = perf_counter() - rust_start

    assert py_result == rust_result, "Results do not match"

    speedup = py_time / rust_time
    console.print(f"Python: {py_time:.6f}s")
    console.print(f"Rust:   {rust_time:.6f}s")
    console.print(f"Speedup: {speedup:.2f}x")


if __name__ == "__main__":
    # Test cases from small to very large
    test_cases = [
        (2, 5),  # Small
        (3, 10),  # Medium
        (4, 15),  # Large
        (5, 20),  # Very large
        (3, 100),  # Wide but shallow
        (10, 3),  # Deep but narrow
    ]

    for depth, width in test_cases:
        benchmark(depth, width)
