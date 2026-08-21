# Code Metrics Analyzer — Dependency-Driven Implementation Order

Feature IDs remain stable traceability identifiers; they are not implementation sequence numbers. The canonical sequence below is a topological order of the hard-prerequisite graph. Stories in the same wave may proceed independently or in parallel.

Repository readiness is satisfied by the closed SETUP-001 work item 343 and the available `rusty-kode` Rust repository.

## Wave 1

1. **MET-003** — Help on empty invocation  
   Prerequisites: None
2. **MET-004** — Version option  
   Prerequisites: None
3. **MET-007** — Standard input  
   Prerequisites: None
4. **MET-009** — Python extension detection  
   Prerequisites: None
5. **MET-010** — Python shebang detection  
   Prerequisites: None
6. **MET-020** — Typed config values  
   Prerequisites: None
7. **MET-029** — IPython magic stripping  
   Prerequisites: None
8. **MET-085** — Source-to-AST conversion  
   Prerequisites: None
9. **MET-087** — Visitor construction from AST  
   Prerequisites: None
10. **MET-090** — Raw Module value object  
   Prerequisites: None
11. **MET-091** — Function/Class value objects  
   Prerequisites: None

## Wave 2

12. **MET-008** — Recursive directory scan  
   Prerequisites: MET-009, MET-010
   - MET-009 supplies Python-extension recognition, which MET-008 consumes in its own acceptance scenarios.
   - MET-010 supplies Python-shebang recognition, which MET-008 consumes in its own acceptance scenarios.
13. **MET-017** — pyproject.toml configuration  
   Prerequisites: MET-020
   - MET-020 supplies typed configuration values, which MET-017 consumes in its own acceptance scenarios.
14. **MET-018** — setup.cfg configuration  
   Prerequisites: MET-020
   - MET-020 supplies typed configuration values, which MET-018 consumes in its own acceptance scenarios.
15. **MET-019** — User home configuration  
   Prerequisites: MET-020
   - MET-020 supplies typed configuration values, which MET-019 consumes in its own acceptance scenarios.
16. **MET-057** — LLOC  
   Prerequisites: MET-090
   - MET-090 supplies the raw-metrics Module value object, which MET-057 consumes in its own acceptance scenarios.
17. **MET-058** — SLOC  
   Prerequisites: MET-090
   - MET-090 supplies the raw-metrics Module value object, which MET-058 consumes in its own acceptance scenarios.
18. **MET-059** — Comment tokens  
   Prerequisites: MET-090
   - MET-090 supplies the raw-metrics Module value object, which MET-059 consumes in its own acceptance scenarios.
19. **MET-060** — Single-comment lines  
   Prerequisites: MET-090
   - MET-090 supplies the raw-metrics Module value object, which MET-060 consumes in its own acceptance scenarios.
20. **MET-061** — Multiline strings  
   Prerequisites: MET-090
   - MET-090 supplies the raw-metrics Module value object, which MET-061 consumes in its own acceptance scenarios.
21. **MET-062** — Blank lines  
   Prerequisites: MET-090
   - MET-090 supplies the raw-metrics Module value object, which MET-062 consumes in its own acceptance scenarios.
22. **MET-067** — Operator and operand counting  
   Prerequisites: MET-085, MET-087
   - MET-085 supplies Python source-to-AST conversion, which MET-067 consumes in its own acceptance scenarios.
   - MET-087 supplies visitor construction and traversal from an AST, which MET-067 consumes in its own acceptance scenarios.
23. **MET-086** — Visitor construction from source  
   Prerequisites: MET-085, MET-087
   - MET-085 supplies Python source-to-AST conversion, which MET-086 consumes in its own acceptance scenarios.
   - MET-087 supplies visitor construction and traversal from an AST, which MET-086 consumes in its own acceptance scenarios.

## Wave 3

24. **MET-006** — Multiple input paths  
   Prerequisites: MET-008
   - MET-008 supplies recursive candidate discovery, which MET-006 consumes in its own acceptance scenarios.
25. **MET-011** — File exclusion patterns  
   Prerequisites: MET-008
   - MET-008 supplies recursive candidate discovery, which MET-011 consumes in its own acceptance scenarios.
26. **MET-012** — Directory ignore patterns  
   Prerequisites: MET-008
   - MET-008 supplies recursive candidate discovery, which MET-012 consumes in its own acceptance scenarios.
27. **MET-013** — Hidden directory default  
   Prerequisites: MET-008
   - MET-008 supplies recursive candidate discovery, which MET-013 consumes in its own acceptance scenarios.
28. **MET-016** — Local project configuration  
   Prerequisites: MET-017, MET-018, MET-019, MET-020
   - MET-017 supplies TOML configuration loading, which MET-016 consumes in its own acceptance scenarios.
   - MET-018 supplies setup.cfg configuration loading, which MET-016 consumes in its own acceptance scenarios.
   - MET-019 supplies user-home configuration loading, which MET-016 consumes in its own acceptance scenarios.
   - MET-020 supplies typed configuration values, which MET-016 consumes in its own acceptance scenarios.
29. **MET-021** — Invalid TOML failure  
   Prerequisites: MET-017
   - MET-017 supplies TOML configuration loading, which MET-021 consumes in its own acceptance scenarios.
30. **MET-026** — Notebook file discovery  
   Prerequisites: MET-008, MET-009
   - MET-008 supplies recursive candidate discovery, which MET-026 consumes in its own acceptance scenarios.
   - MET-009 supplies Python-extension recognition, which MET-026 consumes in its own acceptance scenarios.
31. **MET-027** — Whole-notebook analysis  
   Prerequisites: MET-029, MET-085, MET-086
   - MET-029 supplies notebook source with IPython magic removed, which MET-027 consumes in its own acceptance scenarios.
   - MET-085 supplies Python source-to-AST conversion, which MET-027 consumes in its own acceptance scenarios.
   - MET-086 supplies visitor construction from source, which MET-027 consumes in its own acceptance scenarios.
32. **MET-030** — Block analysis  
   Prerequisites: MET-085, MET-086, MET-087, MET-091
   - MET-085 supplies Python source-to-AST conversion, which MET-030 consumes in its own acceptance scenarios.
   - MET-086 supplies visitor construction from source, which MET-030 consumes in its own acceptance scenarios.
   - MET-087 supplies visitor construction and traversal from an AST, which MET-030 consumes in its own acceptance scenarios.
   - MET-091 supplies Function and Class result value objects, which MET-030 consumes in its own acceptance scenarios.
33. **MET-056** — LOC  
   Prerequisites: MET-058, MET-060, MET-061, MET-062
   - MET-058 supplies the SLOC measurement, which MET-056 consumes in its own acceptance scenarios.
   - MET-060 supplies single-comment-line classification, which MET-056 consumes in its own acceptance scenarios.
   - MET-061 supplies multiline-string-line classification, which MET-056 consumes in its own acceptance scenarios.
   - MET-062 supplies blank-line classification, which MET-056 consumes in its own acceptance scenarios.
34. **MET-068** — Full metric report  
   Prerequisites: MET-067
   - MET-067 supplies Halstead operator and operand counts, which MET-068 consumes in its own acceptance scenarios.

## Wave 4

35. **MET-014** — Custom source encoding  
   Prerequisites: MET-006
   - MET-006 supplies the normalized multi-input stream, which MET-014 consumes in its own acceptance scenarios.
36. **MET-015** — Explicit configuration-file override  
   Prerequisites: MET-016
   - MET-016 supplies the composed project-configuration resolution path, which MET-015 consumes in its own acceptance scenarios.
37. **MET-023** — Per-file error isolation  
   Prerequisites: MET-006
   - MET-006 supplies the normalized multi-input stream, which MET-023 consumes in its own acceptance scenarios.
38. **MET-028** — Per-cell reports  
   Prerequisites: MET-027
   - MET-027 supplies the behavior accepted by MET-027, which MET-028 consumes in its own acceptance scenarios.
39. **MET-031** — A–F ranking  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-031 consumes in its own acceptance scenarios.
40. **MET-032** — If and conditional expressions  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-032 consumes in its own acceptance scenarios.
41. **MET-033** — Boolean operators  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-033 consumes in its own acceptance scenarios.
42. **MET-034** — Try/except/else/finally  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-034 consumes in its own acceptance scenarios.
43. **MET-035** — For/while/async-for  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-035 consumes in its own acceptance scenarios.
44. **MET-036** — Comprehensions and generators  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-036 consumes in its own acceptance scenarios.
45. **MET-037** — Pattern matching  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-037 consumes in its own acceptance scenarios.
46. **MET-038** — Assert decisions  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-038 consumes in its own acceptance scenarios.
47. **MET-039** — With statements  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-039 consumes in its own acceptance scenarios.
48. **MET-040** — Lambda handling  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-040 consumes in its own acceptance scenarios.
49. **MET-041** — Async function handling  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-041 consumes in its own acceptance scenarios.
50. **MET-042** — Class complexity  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-042 consumes in its own acceptance scenarios.
51. **MET-043** — Nested closures  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-043 consumes in its own acceptance scenarios.
52. **MET-044** — Inner classes  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-044 consumes in its own acceptance scenarios.
53. **MET-046** — Show numeric score  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-046 consumes in its own acceptance scenarios.
54. **MET-063** — Per-file terminal report  
   Prerequisites: MET-056, MET-057, MET-058, MET-059, MET-060, MET-061, MET-062
   - MET-056 supplies the composite LOC result, which MET-063 consumes in its own acceptance scenarios.
   - MET-057 supplies the LLOC measurement, which MET-063 consumes in its own acceptance scenarios.
   - MET-058 supplies the SLOC measurement, which MET-063 consumes in its own acceptance scenarios.
   - MET-059 supplies comment-token and comment-percentage inputs, which MET-063 consumes in its own acceptance scenarios.
   - MET-060 supplies single-comment-line classification, which MET-063 consumes in its own acceptance scenarios.
   - MET-061 supplies multiline-string-line classification, which MET-063 consumes in its own acceptance scenarios.
   - MET-062 supplies blank-line classification, which MET-063 consumes in its own acceptance scenarios.
55. **MET-069** — Whole-file report  
   Prerequisites: MET-068
   - MET-068 supplies derived Halstead metric values, which MET-069 consumes in its own acceptance scenarios.
56. **MET-070** — Top-level function reports  
   Prerequisites: MET-068, MET-091
   - MET-068 supplies derived Halstead metric values, which MET-070 consumes in its own acceptance scenarios.
   - MET-091 supplies Function and Class result value objects, which MET-070 consumes in its own acceptance scenarios.
57. **MET-076** — MI calculation  
   Prerequisites: MET-030, MET-057, MET-059, MET-068
   - MET-030 supplies cyclomatic-complexity block results, which MET-076 consumes in its own acceptance scenarios.
   - MET-057 supplies the LLOC measurement, which MET-076 consumes in its own acceptance scenarios.
   - MET-059 supplies comment-token and comment-percentage inputs, which MET-076 consumes in its own acceptance scenarios.
   - MET-068 supplies derived Halstead metric values, which MET-076 consumes in its own acceptance scenarios.
58. **MET-088** — Average complexity helper  
   Prerequisites: MET-030
   - MET-030 supplies cyclomatic-complexity block results, which MET-088 consumes in its own acceptance scenarios.
59. **MET-089** — Complexity sorting helper  
   Prerequisites: MET-030, MET-091
   - MET-030 supplies cyclomatic-complexity block results, which MET-089 consumes in its own acceptance scenarios.
   - MET-091 supplies Function and Class result value objects, which MET-089 consumes in its own acceptance scenarios.

## Wave 5

60. **MET-001** — Console command  
   Prerequisites: MET-063
   - MET-063 supplies one real registered analysis subcommand, allowing MET-001 to verify installed dispatch without waiting for every command story.
61. **MET-024** — Result caching  
   Prerequisites: MET-023
   - MET-023 supplies isolated per-file results and failures, which MET-024 consumes in its own acceptance scenarios.
62. **MET-045** — Rank filtering  
   Prerequisites: MET-031
   - MET-031 supplies complexity rank calculation, which MET-045 consumes in its own acceptance scenarios.
63. **MET-047** — Sort by score  
   Prerequisites: MET-030, MET-089
   - MET-030 supplies cyclomatic-complexity block results, which MET-047 consumes in its own acceptance scenarios.
   - MET-089 supplies the complexity sorting helper, which MET-047 consumes in its own acceptance scenarios.
64. **MET-048** — Sort by lines  
   Prerequisites: MET-030, MET-089
   - MET-030 supplies cyclomatic-complexity block results, which MET-048 consumes in its own acceptance scenarios.
   - MET-089 supplies the complexity sorting helper, which MET-048 consumes in its own acceptance scenarios.
65. **MET-049** — Sort alphabetically  
   Prerequisites: MET-030, MET-089
   - MET-030 supplies cyclomatic-complexity block results, which MET-049 consumes in its own acceptance scenarios.
   - MET-089 supplies the complexity sorting helper, which MET-049 consumes in its own acceptance scenarios.
66. **MET-050** — Filtered average  
   Prerequisites: MET-030, MET-088
   - MET-030 supplies cyclomatic-complexity block results, which MET-050 consumes in its own acceptance scenarios.
   - MET-088 supplies the average-complexity helper, which MET-050 consumes in its own acceptance scenarios.
67. **MET-051** — Total average  
   Prerequisites: MET-030, MET-088
   - MET-030 supplies cyclomatic-complexity block results, which MET-051 consumes in its own acceptance scenarios.
   - MET-088 supplies the average-complexity helper, which MET-051 consumes in its own acceptance scenarios.
68. **MET-064** — Aggregate summary  
   Prerequisites: MET-063
   - MET-063 supplies the first real registered analysis command and terminal raw report, which MET-064 consumes in its own acceptance scenarios.
69. **MET-071** — Async function reports  
   Prerequisites: MET-070
   - MET-070 supplies function-level Halstead results, which MET-071 consumes in its own acceptance scenarios.
70. **MET-077** — Empty/trivial source handling  
   Prerequisites: MET-076
   - MET-076 supplies the Maintainability Index score, which MET-077 consumes in its own acceptance scenarios.
71. **MET-078** — Comment treatment  
   Prerequisites: MET-059, MET-076
   - MET-059 supplies comment-token and comment-percentage inputs, which MET-078 consumes in its own acceptance scenarios.
   - MET-076 supplies the Maintainability Index score, which MET-078 consumes in its own acceptance scenarios.
72. **MET-079** — A–C ranking  
   Prerequisites: MET-076
   - MET-076 supplies the Maintainability Index score, which MET-079 consumes in its own acceptance scenarios.
73. **MET-093** — Maximum complexity threshold  
   Prerequisites: MET-030, MET-031
   - MET-030 supplies cyclomatic-complexity block results, which MET-093 consumes in its own acceptance scenarios.
   - MET-031 supplies complexity rank calculation, which MET-093 consumes in its own acceptance scenarios.

## Wave 6

74. **MET-002** — Runtime-native module invocation  
   Prerequisites: MET-001
   - MET-001 supplies the installed CLI dispatch path, which MET-002 consumes in its own acceptance scenarios.
75. **MET-005** — Top-level error reporting  
   Prerequisites: MET-001
   - MET-001 supplies the installed CLI dispatch path, which MET-005 consumes in its own acceptance scenarios.
76. **MET-022** — Write output to file  
   Prerequisites: MET-001
   - MET-001 supplies the installed CLI dispatch path, which MET-022 consumes in its own acceptance scenarios.
77. **MET-025** — Terminal color control  
   Prerequisites: MET-001
   - MET-001 supplies the installed CLI dispatch path, which MET-025 consumes in its own acceptance scenarios.
78. **MET-052** — JSON export  
   Prerequisites: MET-001, MET-030
   - MET-001 supplies the installed CLI dispatch path, which MET-052 consumes in its own acceptance scenarios.
   - MET-030 supplies cyclomatic-complexity block results, which MET-052 consumes in its own acceptance scenarios.
79. **MET-053** — CCM XML export  
   Prerequisites: MET-001, MET-030
   - MET-001 supplies the installed CLI dispatch path, which MET-053 consumes in its own acceptance scenarios.
   - MET-030 supplies cyclomatic-complexity block results, which MET-053 consumes in its own acceptance scenarios.
80. **MET-054** — Markdown export  
   Prerequisites: MET-001, MET-030
   - MET-001 supplies the installed CLI dispatch path, which MET-054 consumes in its own acceptance scenarios.
   - MET-030 supplies cyclomatic-complexity block results, which MET-054 consumes in its own acceptance scenarios.
81. **MET-055** — Code Climate issue export  
   Prerequisites: MET-001, MET-030, MET-031
   - MET-001 supplies the installed CLI dispatch path, which MET-055 consumes in its own acceptance scenarios.
   - MET-030 supplies cyclomatic-complexity block results, which MET-055 consumes in its own acceptance scenarios.
   - MET-031 supplies complexity rank calculation, which MET-055 consumes in its own acceptance scenarios.
82. **MET-065** — JSON export  
   Prerequisites: MET-001, MET-063, MET-064
   - MET-001 supplies the installed CLI dispatch path, which MET-065 consumes in its own acceptance scenarios.
   - MET-063 supplies the first real registered analysis command and terminal raw report, which MET-065 consumes in its own acceptance scenarios.
   - MET-064 supplies aggregate raw-report values, which MET-065 consumes in its own acceptance scenarios.
83. **MET-066** — XML export  
   Prerequisites: MET-001, MET-063
   - MET-001 supplies the installed CLI dispatch path, which MET-066 consumes in its own acceptance scenarios.
   - MET-063 supplies the first real registered analysis command and terminal raw report, which MET-066 consumes in its own acceptance scenarios.
84. **MET-072** — Terminal total mode  
   Prerequisites: MET-001, MET-069
   - MET-001 supplies the installed CLI dispatch path, which MET-072 consumes in its own acceptance scenarios.
   - MET-069 supplies whole-file Halstead results, which MET-072 consumes in its own acceptance scenarios.
85. **MET-073** — Terminal function mode  
   Prerequisites: MET-001, MET-070, MET-071
   - MET-001 supplies the installed CLI dispatch path, which MET-073 consumes in its own acceptance scenarios.
   - MET-070 supplies function-level Halstead results, which MET-073 consumes in its own acceptance scenarios.
   - MET-071 supplies async-function Halstead results, which MET-073 consumes in its own acceptance scenarios.
86. **MET-074** — JSON export  
   Prerequisites: MET-001, MET-068
   - MET-001 supplies the installed CLI dispatch path, which MET-074 consumes in its own acceptance scenarios.
   - MET-068 supplies derived Halstead metric values, which MET-074 consumes in its own acceptance scenarios.
87. **MET-075** — XML/Markdown export  
   Prerequisites: MET-001, MET-068
   - MET-001 supplies the installed CLI dispatch path, which MET-075 consumes in its own acceptance scenarios.
   - MET-068 supplies derived Halstead metric values, which MET-075 consumes in its own acceptance scenarios.
88. **MET-080** — Rank filtering  
   Prerequisites: MET-001, MET-079
   - MET-001 supplies the installed CLI dispatch path, which MET-080 consumes in its own acceptance scenarios.
   - MET-079 supplies the Maintainability Index rank, which MET-080 consumes in its own acceptance scenarios.
89. **MET-081** — Show numeric MI  
   Prerequisites: MET-001, MET-076
   - MET-001 supplies the installed CLI dispatch path, which MET-081 consumes in its own acceptance scenarios.
   - MET-076 supplies the Maintainability Index score, which MET-081 consumes in its own acceptance scenarios.
90. **MET-082** — Sort by MI  
   Prerequisites: MET-001, MET-076
   - MET-001 supplies the installed CLI dispatch path, which MET-082 consumes in its own acceptance scenarios.
   - MET-076 supplies the Maintainability Index score, which MET-082 consumes in its own acceptance scenarios.
91. **MET-083** — JSON export  
   Prerequisites: MET-001, MET-076
   - MET-001 supplies the installed CLI dispatch path, which MET-083 consumes in its own acceptance scenarios.
   - MET-076 supplies the Maintainability Index score, which MET-083 consumes in its own acceptance scenarios.
92. **MET-084** — XML export  
   Prerequisites: MET-001, MET-076
   - MET-001 supplies the installed CLI dispatch path, which MET-084 consumes in its own acceptance scenarios.
   - MET-076 supplies the Maintainability Index score, which MET-084 consumes in its own acceptance scenarios.
93. **MET-092** — Plugin registration  
   Prerequisites: MET-093
   - MET-093 supplies the Flake8 complexity checker and threshold behavior, which MET-092 consumes in its own acceptance scenarios.
94. **MET-094** — Ignore asserts  
   Prerequisites: MET-038, MET-093
   - MET-038 supplies assert-decision handling, which MET-094 consumes in its own acceptance scenarios.
   - MET-093 supplies the Flake8 complexity checker and threshold behavior, which MET-094 consumes in its own acceptance scenarios.
95. **MET-095** — Show closures  
   Prerequisites: MET-043, MET-093
   - MET-043 supplies nested-closure visibility, which MET-095 consumes in its own acceptance scenarios.
   - MET-093 supplies the Flake8 complexity checker and threshold behavior, which MET-095 consumes in its own acceptance scenarios.

## Wave 7

96. **MET-096** — Threshold configuration  
   Prerequisites: MET-055
   - MET-055 supplies Code Climate-compatible complexity issues, which MET-096 consumes in its own acceptance scenarios.

## Wave 8

97. **MET-097** — Python version selection  
   Prerequisites: MET-096
   - MET-096 supplies the Code Climate engine execution path, which MET-097 consumes in its own acceptance scenarios.
98. **MET-098** — Encoding configuration  
   Prerequisites: MET-096
   - MET-096 supplies the Code Climate engine execution path, which MET-098 consumes in its own acceptance scenarios.
99. **MET-099** — Included paths  
   Prerequisites: MET-096
   - MET-096 supplies the Code Climate engine execution path, which MET-099 consumes in its own acceptance scenarios.
100. **MET-100** — Empty workspace handling  
   Prerequisites: MET-096
   - MET-096 supplies the Code Climate engine execution path, which MET-100 consumes in its own acceptance scenarios.
101. **MET-101** — Stable issue fingerprint  
   Prerequisites: MET-055, MET-096
   - MET-055 supplies Code Climate-compatible complexity issues, which MET-101 consumes in its own acceptance scenarios.
   - MET-096 supplies the Code Climate engine execution path, which MET-101 consumes in its own acceptance scenarios.
102. **MET-102** — Remediation points  
   Prerequisites: MET-055, MET-096
   - MET-055 supplies Code Climate-compatible complexity issues, which MET-102 consumes in its own acceptance scenarios.
   - MET-096 supplies the Code Climate engine execution path, which MET-102 consumes in its own acceptance scenarios.
