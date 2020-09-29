#include <cstdio>
#include <cstdlib>
#include <string>
#include <vector>
#include <stack>
#include <map>
#include <fstream>

#include <unistd.h>

using namespace std;

class Program
{
    string code;
    vector<size_t> bracket_pc;

public:
    Program(const string &source)
    {
        this->code.reserve(source.size());
        this->bracket_pc.resize(source.size(), 0);

        stack<size_t> stack;

        for (char ch : source)
        {
            if (ch == '[')
            {
                stack.push(this->code.size());
            }
            else if (ch == ']')
            {
                size_t left = stack.top();
                stack.pop();
                size_t right = this->code.size();
                this->bracket_pc[left] = right;
                this->bracket_pc[right] = left;
            }
            else
            {
                if (ch != '<' && ch != '>' && ch != '+' && ch != '-' && ch != '.')
                {
                    continue;
                }
            }

            this->code.push_back(ch);
        }
    }

    void run()
    {
        vector<unsigned char> tape{0};
        tape.reserve(8196);

        size_t pc = 0;
        size_t ptr = 0;

        size_t end = this->code.size();

        while (pc < end)
        {
            char ch = this->code[pc];
            if (ch == '+')
            {
                tape[ptr] += 1;
            }
            else if (ch == '-')
            {
                tape[ptr] -= 1;
            }
            else if (ch == '>')
            {
                ptr += 1;
                if (tape.size() == ptr)
                {
                    tape.push_back(0);
                }
            }
            else if (ch == '<')
            {
                ptr -= 1;
            }
            else if (ch == '.')
            {
                putchar(tape[ptr]);
                // printf("%c", tape[ptr]);
            }
            else if (ch == '[' && tape[ptr] == 0)
            {
                pc = this->bracket_pc[pc];
            }
            else if (ch == ']' && tape[ptr] != 0)
            {
                pc = this->bracket_pc[pc];
            }
            pc += 1;
        }
    }
};

int main(int argc, char *argv[])
{
    std::ifstream source_file(getenv("GM_BF_FILE"));

    if (!source_file.is_open()) {
        return -1;
    }

    source_file.seekg(0, std::ios::end);

    string source;

    size_t size = source_file.tellg();
    source.resize(size);
    source_file.seekg(0);
    source_file.read(&source[0], size);

    Program program(source);

    program.run();

    return 0;
}
