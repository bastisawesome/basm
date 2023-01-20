from parser import Parser


def main():
    parser: Parser = Parser()
    print(parser.parse_file("../test.z80"))


if __name__ == "__main__":
    main()
