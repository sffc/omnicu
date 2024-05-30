// generated by diplomat-tool

part of 'lib.g.dart';

/// See the [Rust documentation for `DecomposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html) for more information.
final class DecomposingNormalizer implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  DecomposingNormalizer._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XDecomposingNormalizer_destroy));

  /// Construct a new ICU4XDecomposingNormalizer instance for NFC
  ///
  /// See the [Rust documentation for `new_nfd`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfd) for more information.
  ///
  /// Throws [Error] on failure.
  factory DecomposingNormalizer.nfd(DataProvider provider) {
    final result = _ICU4XDecomposingNormalizer_create_nfd(provider._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DecomposingNormalizer._fromFfi(result.union.ok, []);
  }

  /// Construct a new ICU4XDecomposingNormalizer instance for NFKC
  ///
  /// See the [Rust documentation for `new_nfkd`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfkd) for more information.
  ///
  /// Throws [Error] on failure.
  factory DecomposingNormalizer.nfkd(DataProvider provider) {
    final result = _ICU4XDecomposingNormalizer_create_nfkd(provider._ffi);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DecomposingNormalizer._fromFfi(result.union.ok, []);
  }

  /// Normalize a string
  ///
  /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8) for more information.
  String normalize(String s) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final write = _Write();
    _ICU4XDecomposingNormalizer_normalize(_ffi, sView.allocIn(temp), sView.length, write._ffi);
    temp.releaseAll();
    return write.finalize();
  }

  /// Check if a string is normalized
  ///
  /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `is_normalized_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8) for more information.
  bool isNormalized(String s) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final result = _ICU4XDecomposingNormalizer_is_normalized(_ffi, sView.allocIn(temp), sView.length);
    temp.releaseAll();
    return result;
  }
}

@meta.ResourceIdentifier('ICU4XDecomposingNormalizer_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XDecomposingNormalizer_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XDecomposingNormalizer_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XDecomposingNormalizer_create_nfd')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDecomposingNormalizer_create_nfd')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XDecomposingNormalizer_create_nfd(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier('ICU4XDecomposingNormalizer_create_nfkd')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDecomposingNormalizer_create_nfkd')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XDecomposingNormalizer_create_nfkd(ffi.Pointer<ffi.Opaque> provider);

@meta.ResourceIdentifier('ICU4XDecomposingNormalizer_normalize')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XDecomposingNormalizer_normalize')
// ignore: non_constant_identifier_names
external void _ICU4XDecomposingNormalizer_normalize(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> sData, int sLength, ffi.Pointer<ffi.Opaque> write);

@meta.ResourceIdentifier('ICU4XDecomposingNormalizer_is_normalized')
@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XDecomposingNormalizer_is_normalized')
// ignore: non_constant_identifier_names
external bool _ICU4XDecomposingNormalizer_is_normalized(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> sData, int sLength);
