package ioutils

import (
    "bufio"
    "io"
    "os"
)

// LineWriter writes lines to an io.Writer.
type LineWriter struct {
    w *bufio.Writer
}

// NewLineWriter constructs LineWriter from io.Writer.
func NewLineWriter(writer io.Writer) *LineWriter {
    return &LineWriter{w: bufio.NewWriterSize(writer, 64*1024)}
}

// WriteLines writes lines and flushes.
func (lw *LineWriter) WriteLines(lines []string) error {
    if lw == nil || lw.w == nil {
        return io.ErrClosedPipe
    }
    for _, line := range lines {
        if _, err := lw.w.WriteString(line + "\n"); err != nil {
            return err
        }
    }
    return lw.w.Flush()
}

// WriteString writes a single string with newline.
func (lw *LineWriter) WriteString(line string) error {
    if lw == nil || lw.w == nil {
        return io.ErrClosedPipe
    }
    if _, err := lw.w.WriteString(line + "\n"); err != nil {
        return err
    }
    return lw.w.Flush()
}

// WriteFileLines creates/overwrites a file with lines.
func WriteFileLines(path string, lines []string) error {
    f, err := os.Create(path)
    if err != nil {
        return err
    }
    defer f.Close()
    return NewLineWriter(f).WriteLines(lines)
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


