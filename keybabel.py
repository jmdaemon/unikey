#!/usr/bin/python

# Imports
from bs4 import BeautifulSoup, Comment
from pprint import pprint
import re
import argparse
import logging

# Show debug output
logging.basicConfig(level=logging.DEBUG)

# Functions
def read_file(fname):
    file = open(fname)
    res  = file.read()
    file.close()
    return res

def strip_comments(soup):
    '''  Strip all xml comments '''
    for elem in soup(text=lambda text: isinstance(text, Comment)):
        elem.extract()
    return soup

def extract_xml_keymaps(soup):
    ''' Extract all the keymaps available in the keyboard layout file '''
    # A single apple keyboard layout file can contain multiple keyboard layouts
    keymaps_xml_soup = soup.find_all(attrs={'index': re.compile(r"\d+")})

    # Store all the additional keyboard layouts
    index = 0
    keymaps_xml = {}
    while (index < len(keymaps_xml_soup)):
        keymaps_xml[index] = keymaps_xml_soup[index]
        index += 1
    return keymaps_xml

def extract_apple_keys(soup):
    keymaps_xml = extract_xml_keymaps(soup)

    inter = [ {
        'index': 0,
        'keys': [ {
            'code': '',
            'output': '',
        }, ]
    }, ]

    i = 0
    # For every keyboard layout
    for index, keymap in keymaps_xml.items():
        # Parse the xml content of the layout
        soup_map = BeautifulSoup(str(keymap), features='lxml')

        # Set the current index of the keyboard layout
        index_data = {
            'index': index,
            'keys': [
            ]
        }
        # Insert the layout into our intermediate structure
        inter.insert(index, index_data)
        # For every element in the keyboard layout starting from the end
        for elm in reversed(soup_map()):
            # Get the element's attributes
            att = elm.attrs
            # If the element contains a code and an output
            if 'code' in att and 'output' in att:
                # Get the code and the output corresponding to a key
                code = att['code']
                output = att['output']
                # Save that in a dictionary
                key_data = {
                    'code': code,
                    'output': output,
                }
                # Create a new keys entry
                key_list = inter[index]['keys']
                # Save the key data for later
                key_list.insert(i, key_data)
                # print(f'\n\nindex: {index}')
                # print(inter)
        i += 1
    return inter


# Parse command line arguments
parser = argparse.ArgumentParser(description='Parse an apple keyboard layout into a unikey layout')
parser.add_argument('keyboard_layout_fp')
args = parser.parse_args()
kb_layout_fp  = args.keyboard_layout_fp

# Keyboard layout is an xml file
kb_layout_contents = read_file(kb_layout_fp)
kb_layout_xml = BeautifulSoup(kb_layout_contents, features='lxml')
kb_layout_xml = strip_comments(kb_layout_xml)

# Display entire keyboard layout file
logging.debug(kb_layout_xml.prettify())

# Display all keyboard maps
inter = extract_apple_keys(kb_layout_xml)


def cli(version, verbose, fname):
    contents = read_file(fname)
    soup = BeautifulSoup(contents, features='lxml')

    # Strip all comments
    for elem in soup(text=lambda text: isinstance(text, Comment)):
        elem.extract()

    # Display entire keyboard layout file
    # print(soup.prettify())

    # Display all keyboard maps
    # keymaps = BeautifulSoup(soup.find_all("keymap"))
    # print(keymaps.prettify())
    inter = extract_apple_keys(soup)

    # layout = [{
        # 'index': 0,
        # 'keymap': {
            # 'row': 'e',
            # 'keys': [{
                # 'code': '',
                # 'output': '',
            # }]
        # }
    # }, ]

    # Apple to Linux XKB Keyboard Mappings
    # Rows are laid out in Linux XKB keymap order
    RowAE = [
        { 'code': '18'  , 'output': '1'},
        { 'code': '19'  , 'output': '2'},
        { 'code': '20'  , 'output': '3'},
        { 'code': '21'  , 'output': '4'},
        { 'code': '23'  , 'output': '5'},
        { 'code': '22'  , 'output': '6'},
        { 'code': '26'  , 'output': '7'},
        { 'code': '28'  , 'output': '8'},
        { 'code': '25'  , 'output': '9'},
        { 'code': '29'  , 'output': '0'},
        { 'code': '27'  , 'output': '-'},
        { 'code': '24'  , 'output': '='}]
    RowAD = [
        { 'code': '12'  , 'output': 'q'},
        { 'code': '13'  , 'output': 'w'},
        { 'code': '14'  , 'output': 'e'},
        { 'code': '15'  , 'output': 'r'},
        { 'code': '17'  , 'output': 't'},
        { 'code': '16'  , 'output': 'y'},
        { 'code': '32'  , 'output': 'u'},
        { 'code': '34'  , 'output': 'i'},
        { 'code': '31'  , 'output': 'o'},
        { 'code': '35'  , 'output': 'p'},
        { 'code': '33'  , 'output': '['},
        { 'code': '30'  , 'output': ']'}]
    RowAC = [
        { 'code': '0'   , 'output': 'a'},
        { 'code': '1'   , 'output': 's'},
        { 'code': '2'   , 'output': 'd'},
        { 'code': '3'   , 'output': 'f'},
        { 'code': '5'   , 'output': 'g'},
        { 'code': '4'   , 'output': 'h'},
        { 'code': '38'  , 'output': 'j'},
        { 'code': '40'  , 'output': 'k'},
        { 'code': '37'  , 'output': 'l'},
        { 'code': '41'  , 'output': ';'},
        { 'code': '39'  , 'output': '\''}]
    RowAB = [
        { 'code': '6'   , 'output': 'z'},
        { 'code': '7'   , 'output': 'x'},
        { 'code': '8'   , 'output': 'c'},
        { 'code': '9'   , 'output': 'v'},
        { 'code': '11'  , 'output': 'b'},
        { 'code': '45'  , 'output': 'n'},
        { 'code': '46'  , 'output': 'm'},
        { 'code': '43'  , 'output': ','},
        { 'code': '47'  , 'output': '.'},
        { 'code': '44'  , 'output': '/'}]
    Special_Keys = [
        {'apple_code': '', 'linux_code': '', 'output': '\\'},
        {'apple_code': '', 'linux_code': '', 'output': '`'}]
    rows = {'e': RowAE, 'd': RowAD, 'c': RowAC, 'b': RowAB}

    # keyboard_layouts = [rows for kmap in inter]
    keyboard_layouts = []
    for kmap in inter:
        keyboard_layouts.append(rows)
    pprint(keyboard_layouts)
    print(len(keyboard_layouts))

    # Apple Keys:
        # Multiple sets of keymaps
            # Each keymap contains a list of key 'codes', and 'outputs'
    # Linux Keys:
        # A single keymap
            # Multiple sets of rows
                # Each row contains multiple sets of keys, alongside their key modifier variants

    # Most naive loop
    # For all apple_keymaps
        # For every keymap in apple_keymaps
            # For every row in keyboard_layouts
                # For all key_modifiers in row
                    # For every key in key_modifier
                        # if linux_key['code'] == apple_key['code'] # If their codes are the same
                        # Convert the apple_key to linux_key
    # We can ditch modifiers for now and do
    # For all apple_keymaps
        # For every keymap in apple_keymaps
            # For every row in keyboard_layouts
                # For every key in rows
                    # if linux_key['code'] == apple_key['code'] # If their codes are the same
                    # Convert the apple_key to linux_key

    # Converting special keys
    # Let Special_Keys = {
    #   {'linux_code': 'BKSL', 'apple_code': '35', 'output': '\\'},
    #   {}
    # }
    # for row in keyboard_layouts:
        # if row == Special_Keys
            # for linux_keys in row:
                # for linux_key in linux_keys
                    # if linux_key['apple_code'] == apple_key['code']
                    # linux_key['apple_code'] = apple_key['output']
    # output 
    # We still have no way to convert special keys just yet.
    # sys.exit()
    # row_index = 0

    # if apple_key['code'] == linux_key['code']
    #       linux_key['output'] = apple_key['output']
    # If the apple key code matches the conversion code listed in the table
    # Set the output of the linux key to the apple key

    # How to properly iterate through keys
    # Iterate through every apple key
    # print(inter[3]['keys'])

    # for row, keys in rows.items():
        # print(f'row: {row}')
    index = 0
    # for apple_key in inter[1]['keys']:
    while index < len(inter):
        print(inter[index]['keys'])
        print(f'Keymap Index: #{index}')
        for apple_key in inter[index]['keys']:
            for rows in keyboard_layouts:
                for row, keys in rows.items():
                    for linux_key in keys:
                        # linux_key = keys[row_index]
                        # print(apple_key['code'], linux_key['code'])
                        if apple_key['code'] == linux_key['code']:
                            key = apple_key['output']
                            print(f'Convert Key: {key}')
                            linux_key['code'] = key

            # for row, keys in rows.items():
                # for linux_key in keys:
                    # # linux_key = keys[row_index]
                    # # print(apple_key['code'], linux_key['code'])
                    # if apple_key['code'] == linux_key['code']:
                        # key = apple_key['output']
                        # print(f'Convert Key: {key}')
        index += 1
    print(keyboard_layouts)
    while index < len(inter):
        for apple_key in inter[index]['keys']:
            for rows in keyboard_layouts:
                for row, keys in rows.items():
                    for linux_key in keys:
                        if apple_key['code'] == ';':
                            pass

                # row_index += 1
            # for linux_key in keys:
                # print(linux_key['code'])
                # if apple_key['code'] == linux_key['code']:
                    # key = apple_key['output']
                    # print(f'Convert Key: {key}')
                    # print("Do something")
                    # row_index += 1
                    # print()
                # print(linux_key)
                # print(apple_key['output'])
        # row_index = 0
        # for key in inter[0]['keys']:
            # if key['code'] == ''

    # for key in inter[0]['keys']:
        # print(key)
        # print(key['code'])
        # print(key['output'])

        # print(key.values())
        # print(key['code'])
        # print(key['output'])
        # print(soup_map["key"])
        # print(f'\n\nindex: {index}\n')
        # for key in keys:
            # print(key.prettify())

    # layout = { { {}, }, }

