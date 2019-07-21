FROM rustlang/rust:nightly
RUN apt-get install git
RUN git clone https://github.com/ioncodes/thicc ~/thicc
RUN cd ~/thicc && cargo build --release
CMD ~/thicc/target/release/thicc
EXPOSE 7000/tcp
