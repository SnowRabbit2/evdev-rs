#!/usr/bin/env python
# Parses linux/input.h scanning for #define KEY_FOO 134
# Prints Rust source header files that can be used for
# mapping and lookup tables.
#
# The original version of this file is in libevdev
#

from __future__ import print_function
import re
import sys

prefixes = [
    "EV",
    "SYN",
    "KEY",
    "BTN",
    "REL",
    "ABS",
    "MSC",
    "SW",
    "LED",
    "SND",
    "REP",
    "FF_STATUS",
    "FF",
    "INPUT_PROP",
    "BUS"
]

matcher = re.compile(r"^#define\s+({})_(\w+)\s+(0x[0-9A-Fa-f]+|[0-9]+)"
        .format("|".join(prefixes)))

blacklist = [
    "EV_VERSION",
    "BTN_MISC",
    "BTN_MOUSE",
    "BTN_JOYSTICK",
    "BTN_GAMEPAD",
    "BTN_DIGI",
    "BTN_WHEEL",
    "BTN_TRIGGER_HAPPY",
    "SW_MAX",
    "REP_MAX",
]

class Constant(object):
    def __init__(self, match):
        self.prefix = match[1]
        self.name = match[2]
        if match[3].startswith("0x"):
            self.value = int(match[3][2:], 16)
        else:
            self.value = int(match[3], 10)

    def c_name(self):
        return "{}_{}".format(self.prefix, self.name)

    def sanitized_name(self):
        if self.name[0].isdigit():
            return "{}_{}".format(self.prefix, self.name)
        else:
            return self.name


class Prefix(object):
    def __init__(self, name):
        self.name = name
        self.max = None
        self.values = {}
        self.done = False


def parse_line(prefixes, line):
    match = matcher.match(line)
    if match == None:
        return

    c = Constant(match)
    if c.c_name() in blacklist:
        return

    if not c.prefix in prefixes:
        prefixes[c.prefix] = Prefix(c.prefix)
    prefix = prefixes[c.prefix]
    if c.name == "MAX":
        prefix.max = c
    else:
        prefix.values[c.value] = c


def parse(fp, prefixes):
    lines = fp.readlines()
    for line in lines:
        parse_line(prefixes, line)

def print_enum_code_type(prefixes):
    prefix = prefixes["EV"]

    print("#[allow(non_camel_case_types)]")
    print("#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]")
    print("pub enum Type {")
    for c in prefix.values.values():
        print("    {},".format(c.name, hex(c.value)))
    print("    UNKNOWN(TypeRaw),")
    print("}")
    print("")

    print("impl Type {")
    print("    pub fn from_raw(r#type: TypeRaw) -> Type {")
    print("        match r#type {")
    for c in prefix.values.values():
        print("            {} => Type::{},".format(hex(c.value), c.name))
    print("            _ => Type::UNKNOWN(r#type),")
    print("        }")
    print("    }")
    print("    pub fn as_raw(&self) -> TypeRaw {")
    print("        match *self {")
    for c in prefix.values.values():
        print("            Type::{} => {},".format(c.name, hex(c.value)))
    print("            Type::UNKNOWN(r#type) => r#type,")
    print("        }")
    print("    }")
    print("}")
    print("")

    print("#[allow(non_camel_case_types)]")
    print("#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]")
    print("pub enum Code {")
    for c in prefix.values.values():
        print("    {}(CodeValRaw),".format(c.name))
    print("    UNKNOWN(TypeRaw, CodeValRaw),")
    print("}")
    print("")

    print("impl Code {")
    print("    pub fn from_raw(r#type: Type, codeval: CodeValRaw) -> Code {")
    print("        let r#type = r#type.as_raw();")
    print("        match r#type {")
    for c in prefix.values.values():
        print("            {} => Code::{}(codeval),".format(hex(c.value), c.name))
    print("            _ => Code::UNKNOWN(r#type, codeval),")
    print("        }")
    print("    }")

    print("    pub fn get_type(&self) -> Type {")
    print("        match *self {")
    for c in prefix.values.values():
        print("            Code::{}(..) => Type::{},".format(c.name, c.name))
    print("            Code::UNKNOWN(r#type, _) => Type::UNKNOWN(r#type),")
    print("        }")
    print("    }")

    print("    pub fn as_raw(&self) -> (TypeRaw, CodeValRaw) {")
    print("        match *self {")
    for c in prefix.values.values():
        print("            Code::{}(codeval) => ({}, codeval),".format(c.name, hex(c.value)))
    print("            Code::UNKNOWN(r#type, codeval) => (r#type, codeval),".format(c.name, hex(c.value)))
    print("        }")
    print("    }")
    print("}")
    print("")

    print("impl AsCodeRaw for (Type, CodeValRaw) {")
    print("    fn as_code_raw(&self) -> CodeRaw {")
    print("        match self.0 {")
    for c in prefix.values.values():
        print("            Type::{} => ({}, self.1),".format(c.name, hex(c.value)))
    print("            Type::UNKNOWN(r#type) => (r#type, self.1),")
    print("        }")
    print("    }")
    print("}")
    print("")

    prefix.done = True

    for c in prefix.values.values():
        print_enum(prefixes.get(c.name))
        print_as_code(c, prefixes.get(c.name))
        if c.name == "KEY":
            print_enum(prefixes.get("BTN"))
            print_as_code(c, prefixes.get("BTN"))


def print_enum(prefix):
    if prefix is None:
        return

    print("#[allow(non_camel_case_types)]")
    print("#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]")
    print("pub enum {} {{".format(prefix.name))
    for c in prefix.values.values():
        print("    {} = {},".format(c.sanitized_name(), hex(c.value)))
    print("}")
    print("")

    prefix.done = True

    print("impl {} {{".format(prefix.name))
    print("    pub fn from_raw(codeval: CodeValRaw) -> Option<{}> {{".format(prefix.name))
    print("        match codeval {")
    for c in prefix.values.values():
        print("            {} => Some({}::{}),".format(hex(c.value), prefix.name, c.sanitized_name()))
    print("            _ => None,")
    print("        }")
    print("    }")
    print("    pub fn as_raw(&self) -> CodeValRaw {{".format(prefix.name))
    print("        *self as CodeValRaw")
    print("    }")
    if prefix.max is not None:
        print("    pub fn max() -> CodeValRaw {{".format(prefix.name))
        print("        {}".format(prefix.max.value))
        print("    }")
    print("}")
    print("")


def print_as_code(type, prefix):
    if prefix is None:
        return

    print("impl AsCodeRaw for {} {{".format(prefix.name))
    print("    fn as_code_raw(&self) -> CodeRaw {")
    print("        ({}, *self as CodeValRaw)".format(hex(type.value)))
    print("    }")
    print("}")
    print("")

    print("impl AsCode for {} {{".format(prefix.name))
    print("    fn as_code(&self) -> Code {")
    print("        Code::{}(*self as CodeValRaw)".format(type.name))
    print("    }")
    print("}")
    print("")


def usage(prog):
    print("Usage: %s /path/to/linux/input.h" % prog)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        usage(sys.argv[0])
        sys.exit(2)

    print("/* THIS FILE IS GENERATED, DO NOT EDIT */")
    print("")

    prefixes = dict()
    for i in (1, len(sys.argv) - 1):
        with open(sys.argv[i]) as file:
            parse(file, prefixes)

    print_enum_code_type(prefixes)
    for prefix in prefixes.values():
        if not prefix.done:
            print_enum(prefix)
