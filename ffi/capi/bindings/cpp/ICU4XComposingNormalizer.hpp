#ifndef ICU4XComposingNormalizer_HPP
#define ICU4XComposingNormalizer_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XComposingNormalizer.h"

class ICU4XDataProvider;
class ICU4XComposingNormalizer;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XComposingNormalizer with std::unique_ptr.
 */
struct ICU4XComposingNormalizerDeleter {
  void operator()(capi::ICU4XComposingNormalizer* l) const noexcept {
    capi::ICU4XComposingNormalizer_destroy(l);
  }
};

/**
 * See the [Rust documentation for `ComposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html) for more information.
 */
class ICU4XComposingNormalizer {
 public:

  /**
   * Construct a new ICU4XComposingNormalizer instance for NFC
   * 
   * See the [Rust documentation for `new_nfc`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfc) for more information.
   */
  static diplomat::result<ICU4XComposingNormalizer, ICU4XError> create_nfc(const ICU4XDataProvider& provider);

  /**
   * Construct a new ICU4XComposingNormalizer instance for NFKC
   * 
   * See the [Rust documentation for `new_nfkc`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfkc) for more information.
   */
  static diplomat::result<ICU4XComposingNormalizer, ICU4XError> create_nfkc(const ICU4XDataProvider& provider);

  /**
   * Normalize a string
   * 
   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
   * to the WHATWG Encoding Standard.
   * 
   * See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8) for more information.
   */
  template<typename W> void normalize_to_write(const std::string_view s, W& write) const;

  /**
   * Normalize a string
   * 
   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
   * to the WHATWG Encoding Standard.
   * 
   * See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8) for more information.
   */
  std::string normalize(const std::string_view s) const;

  /**
   * Check if a string is normalized
   * 
   * Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
   * to the WHATWG Encoding Standard.
   * 
   * See the [Rust documentation for `is_normalized_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8) for more information.
   */
  bool is_normalized(const std::string_view s) const;
  inline const capi::ICU4XComposingNormalizer* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XComposingNormalizer* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XComposingNormalizer(capi::ICU4XComposingNormalizer* i) : inner(i) {}
  ICU4XComposingNormalizer() = default;
  ICU4XComposingNormalizer(ICU4XComposingNormalizer&&) noexcept = default;
  ICU4XComposingNormalizer& operator=(ICU4XComposingNormalizer&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XComposingNormalizer, ICU4XComposingNormalizerDeleter> inner;
};

#include "ICU4XDataProvider.hpp"

inline diplomat::result<ICU4XComposingNormalizer, ICU4XError> ICU4XComposingNormalizer::create_nfc(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XComposingNormalizer_create_nfc(provider.AsFFI());
  diplomat::result<ICU4XComposingNormalizer, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XComposingNormalizer>(ICU4XComposingNormalizer(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XComposingNormalizer, ICU4XError> ICU4XComposingNormalizer::create_nfkc(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XComposingNormalizer_create_nfkc(provider.AsFFI());
  diplomat::result<ICU4XComposingNormalizer, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XComposingNormalizer>(ICU4XComposingNormalizer(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline void ICU4XComposingNormalizer::normalize_to_write(const std::string_view s, W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::ICU4XComposingNormalizer_normalize(this->inner.get(), s.data(), s.size(), &write_writer);
}
inline std::string ICU4XComposingNormalizer::normalize(const std::string_view s) const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::ICU4XComposingNormalizer_normalize(this->inner.get(), s.data(), s.size(), &diplomat_write_out);
  return diplomat_write_string;
}
inline bool ICU4XComposingNormalizer::is_normalized(const std::string_view s) const {
  return capi::ICU4XComposingNormalizer_is_normalized(this->inner.get(), s.data(), s.size());
}
#endif
