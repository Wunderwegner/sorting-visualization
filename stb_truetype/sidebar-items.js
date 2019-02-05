initSidebarItems({"enum":[["MacEid",""],["MacLang",""],["MicrosoftEid",""],["MicrosoftLang",""],["PlatformEncodingLanguageId",""],["PlatformId",""],["UnicodeEid",""],["VertexType",""]],"fn":[["get_font_offset_for_index","Each .ttf/.ttc file may have more than one font. Each font has a sequential index number starting from 0. Call this function to get the font offset for a given index; it returns None if the index is out of range. A regular .ttf file will only define one font and it always be at offset 0, so it will return Some(0) for index 0, and None for all other indices. You can just skip this step if you know it's that kind of font."],["is_collection","Return `true` if `data` holds a TrueType Collection, according to its signature in the initial bytes. A TrueType Collection stores several fonts in a single file, allowing them to share data for glyphs they have in common."],["is_font","Return `true` if `data` holds a font stored in a format this crate recognizes, according to its signature in the initial bytes."]],"struct":[["FontInfo",""],["FontNameIter",""],["HMetrics",""],["Rect",""],["VMetrics",""],["Vertex",""]]});