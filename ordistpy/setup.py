from setuptools import setup

install_requires=["numpy"]
packages=["ordist"]

setup(
    name="ordist",
    version="0.1.1",
    license="Apache-2.0",
    description="Order Distance",
    
    packages=packages,
    install_requires=install_requires,
    include_package_data=True,

    author="Shuntaro Ohno",
    author_email="shuntaro94@gmail.com",
    url="ushitora.com",
)