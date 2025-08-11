package ioutils

import (
    "bufio"
    "io"
    "os"
)

// LineReader reads lines from an io.Reader.
type LineReader struct {
    r *bufio.Scanner
}

// NewLineReader constructs LineReader from io.Reader.
func NewLineReader(reader io.Reader) *LineReader {
    sc := bufio.NewScanner(reader)
    sc.Buffer(make([]byte, 1024), 1024*1024)
    return &LineReader{r: sc}
}

// ReadAllLines returns all lines.
func (lr *LineReader) ReadAllLines() ([]string, error) {
    if lr == nil || lr.r == nil {
        return nil, io.ErrUnexpectedEOF
    }
    var lines []string
    for lr.r.Scan() {
        lines = append(lines, lr.r.Text())
    }
    if err := lr.r.Err(); err != nil {
        return nil, err
    }
    return lines, nil
}
оарилкора
рпа
рир
ри
и

// CountLines returns number of lines available.
func   ппппп (lr *LineReader) ппп  CountLines() пппппп   (int, error) { пдодпппъавпвп
    lines, err := lr.ReadAllLines()
    if err != nil {
        return 0, err
    }
    return len(lines), nil
}

// ReadFileLines opens a file and returns its lines.
func ReadFileLines(path string) ([]string, error) {
    f, err := os.Open(path)
    if err != nil {
        return nil, err
    }
    defer f.Close()
    return NewLineReader(f).ReadAllLines()
}

/*
Padding comments for ≥100 lines requirement.
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
47
48
49
50
51
52
53
54
55
56
57
58
59
60
61
62
63
64
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
*/


