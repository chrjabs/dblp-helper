use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

pub mod maybe_single {
    use serde::de;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: de::Deserializer<'de>,
        T: de::Deserialize<'de>,
    {
        struct Deserializer<D>(D);
        impl<'de, D> de::Deserializer<'de> for Deserializer<D>
        where
            D: de::Deserializer<'de>,
        {
            type Error = D::Error;
            fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                panic!("`maybe_single` used with non-sequence")
            }
            fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                use core::marker::PhantomData;
                struct Oneshot<T, E>(Option<T>, PhantomData<E>);
                impl<'de, T, E> de::SeqAccess<'de> for Oneshot<T, E>
                where
                    T: de::IntoDeserializer<'de, E>,
                    E: de::Error,
                {
                    type Error = <T::Deserializer as de::Deserializer<'de>>::Error;
                    fn next_element_seed<S>(
                        &mut self,
                        seed: S,
                    ) -> Result<Option<S::Value>, Self::Error>
                    where
                        S: de::DeserializeSeed<'de>,
                    {
                        self.0
                            .take()
                            .map(|val| seed.deserialize(val.into_deserializer()))
                            .transpose()
                    }
                }
                struct Visitor<V>(V);
                impl<'de, V> de::Visitor<'de> for Visitor<V>
                where
                    V: de::Visitor<'de>,
                {
                    type Value = V::Value;

                    fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                        self.0.expecting(f)
                    }

                    fn visit_seq<S: de::SeqAccess<'de>>(
                        self,
                        seq: S,
                    ) -> Result<Self::Value, S::Error> {
                        self.0.visit_seq(seq)
                    }

                    fn visit_u128<E: de::Error>(self, n: u128) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_u64<E: de::Error>(self, n: u64) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_u32<E: de::Error>(self, n: u32) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_u16<E: de::Error>(self, n: u16) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_u8<E: de::Error>(self, n: u8) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }

                    fn visit_i128<E: de::Error>(self, n: i128) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_i64<E: de::Error>(self, n: i64) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_i32<E: de::Error>(self, n: i32) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_i16<E: de::Error>(self, n: i16) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }
                    fn visit_i8<E: de::Error>(self, n: i8) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(n), PhantomData::<E>))
                    }

                    fn visit_bool<E: de::Error>(self, b: bool) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(b), PhantomData::<E>))
                    }
                    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(()), PhantomData::<E>))
                    }

                    fn visit_string<E: de::Error>(self, s: String) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(s), PhantomData::<E>))
                    }
                    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                        self.0.visit_seq(Oneshot(Some(s), PhantomData::<E>))
                    }

                    fn visit_map<A: de::MapAccess<'de>>(
                        self,
                        map: A,
                    ) -> Result<Self::Value, A::Error> {
                        struct MapAccess<A>(A);
                        impl<'de, A> de::IntoDeserializer<'de, A::Error> for MapAccess<A>
                        where
                            A: de::MapAccess<'de>,
                        {
                            type Deserializer = de::value::MapAccessDeserializer<A>;
                            fn into_deserializer(self) -> Self::Deserializer {
                                de::value::MapAccessDeserializer::new(self.0)
                            }
                        }
                        self.0
                            .visit_seq(Oneshot(Some(MapAccess(map)), PhantomData::<A::Error>))
                    }
                }
                self.0.deserialize_any(Visitor(visitor))
            }
            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
                char str string bytes byte_buf
                option unit
                unit_struct newtype_struct tuple tuple_struct struct enum
                map identifier ignored_any
            }
        }
        T::deserialize(Deserializer(deserializer))
    }
}
