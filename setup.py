from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

PKG_NAME = "unformat"

with open(f"{PKG_NAME}/__init__.py", encoding="utf-8") as f:
    for line in f:
        if line.startswith("__version__"):
            VERSION = line.strip().split()[-1][1:-1]
            break

with open("README.md", "r") as f:
    readme = f.read()
    
setup(
    name=PKG_NAME,
    description="Python string unformatter",
    long_description=readme,
    long_description_content_type="text/markdown",
    version=VERSION,
    license="MIT",
    packages=find_packages(exclude=["tests", "tests.*"]),
    rust_extensions=[
        RustExtension("#####", "Cargo.toml", binding=Binding.PyO3)  # TODO
    ],
    install_requires=[],
    setup_requires=["setuptools_rust>=1.5.2"],
    python_requires=">=3.8",
)
