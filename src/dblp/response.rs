use std::fmt;

use owo_colors::OwoColorize;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Response {
    result: Result,
}

impl Response {
    pub fn iter_hits(&self) -> std::slice::Iter<Hit> {
        self.result.hits.hits.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Result {
    completions: Completions,
    hits: Hits,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Completions {
    #[serde(rename = "@total")]
    total: String,
    #[serde(rename = "c", with = "maybe_single")]
    completions: Vec<Completion>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Completion {
    text: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Hits {
    #[serde(rename = "@total")]
    total: String,
    #[serde(rename = "hit", with = "maybe_single")]
    hits: Vec<Hit>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Hit {
    info: Info,
}

impl Hit {
    pub fn display(&self) -> HitDisplay<'_> {
        HitDisplay {
            value: self,
            styles: Box::default(),
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
#[serde_with::skip_serializing_none]
struct Info {
    authors: Authors,
    title: String,
    year: String,
    r#type: String,
    access: String,
    key: String,
    ee: String,
    url: String,
    venue: Option<String>,
    pages: Option<String>,
    doi: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Authors {
    #[serde(with = "maybe_single")]
    author: Vec<Author>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Author {
    #[serde(rename = "@pid")]
    pid: String,
    text: String,
}

/// Displayer for [`Hit`]
pub struct HitDisplay<'a> {
    value: &'a Hit,
    styles: Box<crate::cli::Styles>,
}

impl HitDisplay<'_> {
    /// Colorizes the output.
    pub fn colorize(&mut self) {
        self.styles.colorize();
    }
}

impl fmt::Display for HitDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            format!("DBLP:{}", self.value.info.key).style(self.styles.citekey)
        )?;
        writeln!(f, "{}", "----------".style(self.styles.separator))?;
        for (idx, author) in self.value.info.authors.author.iter().enumerate() {
            write!(f, "{}", author.text.style(self.styles.authors))?;
            if idx < self.value.info.authors.author.len() {
                write!(f, ", ")?;
            }
        }
        writeln!(f)?;
        writeln!(f, "{}", self.value.info.title.style(self.styles.title))?;
        if let Some(venue) = &self.value.info.venue {
            write!(f, "{}", venue.style(self.styles.venue))?;
            write!(f, ", ")?;
        }
        writeln!(f, "({})", self.value.info.year.style(self.styles.year))?;
        writeln!(f, "{}", self.value.info.ee.style(self.styles.url))?;
        Ok(())
    }
}

mod maybe_single {
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

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        let data = r#"
            {
            "result":{
            "query":"test*",
            "status":{
            "@code":"200",
            "text":"OK"
            },
            "time":{
            "@unit":"msecs",
            "text":"2.93"
            },
            "completions":{
            "@total":"498",
            "@computed":"100",
            "@sent":"2",
            "c":[
            {
            "@sc":"48074",
            "@dc":"44805",
            "@oc":"48074",
            "@id":"59637089",
            "text":"test"
            },
            {
            "@sc":"41326",
            "@dc":"40495",
            "@oc":"41326",
            "@id":"59637375",
            "text":"testing"
            }
            ]
            },
            "hits":{
            "@total":"99047",
            "@computed":"100",
            "@sent":"1",
            "@first":"1",
            "hit":[{
            "@score":"5",
            "@id":"6425006",
            "info":{"authors":{"author":[{"@pid":"28/2391","text":"Tsuyoshi Shinogi"},{"@pid":"91/2238","text":"Hiroyuki Yamada"},{"@pid":"56/844","text":"Terumine Hayashi"},{"@pid":"21/5484","text":"Shinji Tsuruoka"},{"@pid":"67/2808","text":"Tomohiro Yoshikawa"}]},"title":"A Test Cost Reduction Method by Test Response and Test Vector Overlapping for Full-Scan Test Architecture.","venue":"Asian Test Symposium","pages":"366-371","year":"2005","type":"Conference and Workshop Papers","access":"closed","key":"conf/ats/ShinogiYHTY05","doi":"10.1109/ATS.2005.17","ee":"https://doi.org/10.1109/ATS.2005.17","url":"https://dblp.org/rec/conf/ats/ShinogiYHTY05"},
            "url":"URL#6425006"
            }
            ]
            }
            }
            }
        "#;

        let v: super::Response = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn single() {
        let data = r#"
            {
            "result":{
            "query":"test*",
            "status":{
            "@code":"200",
            "text":"OK"
            },
            "time":{
            "@unit":"msecs",
            "text":"2.93"
            },
            "completions":{
            "@total":"498",
            "@computed":"100",
            "@sent":"2",
            "c":[
            {
            "@sc":"48074",
            "@dc":"44805",
            "@oc":"48074",
            "@id":"59637089",
            "text":"test"
            },
            {
            "@sc":"41326",
            "@dc":"40495",
            "@oc":"41326",
            "@id":"59637375",
            "text":"testing"
            }
            ]
            },
            "hits":{
            "@total":"99047",
            "@computed":"100",
            "@sent":"1",
            "@first":"1",
            "hit":[{
            "@score":"5",
            "@id":"6425006",
            "info":{"authors":{"author":{"@pid":"28/2391","text":"Tsuyoshi Shinogi"}},"title":"A Test Cost Reduction Method by Test Response and Test Vector Overlapping for Full-Scan Test Architecture.","venue":"Asian Test Symposium","pages":"366-371","year":"2005","type":"Conference and Workshop Papers","access":"closed","key":"conf/ats/ShinogiYHTY05","doi":"10.1109/ATS.2005.17","ee":"https://doi.org/10.1109/ATS.2005.17","url":"https://dblp.org/rec/conf/ats/ShinogiYHTY05"},
            "url":"URL#6425006"
            }
            ]
            }
            }
            }
        "#;

        let v: super::Response = serde_json::from_str(data).unwrap();
    }
}
