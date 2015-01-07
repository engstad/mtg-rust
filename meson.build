project('mtg-rust', 'rust')

l = static_library('mtg', ['src/interval.rs'], install : true)
e = executable('main', 'main.rs', link_with : l, install : true)
test('mtg-link', e)
