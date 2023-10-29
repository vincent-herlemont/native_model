use bincode::{config, Decode, Encode};
use native_model::Result;
use native_model::{DecodeBodyError, DecodeResult, EncodeBodyError, EncodeResult, Model};
// Add this function to the macro for custom serialization
fn native_model_encode<T: Encode>(obj: &T) -> anyhow::Result<Vec<u8>> {
    let result = bincode::encode_to_vec(obj, config::standard())?;
    Ok(result)
}

// Add this function to the macro for custom deserialization
fn native_model_decode<T: Decode>(data: Vec<u8>) -> anyhow::Result<T> {
    let (result, _) =
        bincode::decode_from_slice(&data, config::standard()).map_err(|e| EncodeBodyError {
            msg: format!("Decode error: {}", e),
            source: e.into(),
        })?;
    Ok(result)
}

#[derive(Debug, Encode, Decode)]
struct A {}
impl Model for A {
    fn native_model_id() -> u32 {
        1
    }

    fn native_model_version() -> u32 {
        1
    }

    fn native_model_decode_upgrade_body(_data: Vec<u8>, _id: u32, version: u32) -> Result<Self> {
        println!(
            "A::deserialization_and_upgrade({}, {})",
            version,
            Self::native_model_version()
        );
        if version == Self::native_model_version() {
            Ok(Self {})
        } else if version < Self::native_model_version() {
            panic!("The version {} not supported", version);
        } else {
            panic!("Not implemented");
        }
    }

    fn native_model_encode_body(&self) -> EncodeResult<Vec<u8>>
    where
        Self: Sized,
    {
        native_model_encode(self).map_err(|e| EncodeBodyError {
            msg: format!("{}", e),
            source: e.into(),
        })
    }

    fn native_model_decode_body(data: Vec<u8>, _id: u32) -> DecodeResult<Self>
    where
        Self: Sized,
    {
        native_model_decode(data).map_err(|e| DecodeBodyError::DecodeError {
            msg: format!("{}", e),
            source: e.into(),
        })
    }

    fn native_model_encode_downgrade_body(self, version: u32) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        println!(
            "A::serialization_and_downgrade({}, {})",
            version,
            Self::native_model_version()
        );
        if version == Self::native_model_version() {
            let result = self.native_model_encode_body()?;
            Ok(result)
        } else if version < Self::native_model_version() {
            panic!("The version {} not supported", version);
        } else {
            panic!("Not implemented");
        }
    }
}

#[derive(Debug, Encode, Decode)]
struct B {}
impl Model for B {
    fn native_model_id() -> u32 {
        1
    }

    fn native_model_version() -> u32 {
        2
    }

    fn native_model_decode_upgrade_body(_data: Vec<u8>, id: u32, version: u32) -> Result<Self> {
        println!(
            "B::deserialization_and_upgrade({}, {})",
            version,
            Self::native_model_version()
        );
        if version == Self::native_model_version() {
            Ok(Self {})
        } else if version < Self::native_model_version() {
            A::native_model_decode_upgrade_body(_data, id, version).map(|a| a.into())
        } else {
            panic!("Not implemented");
        }
    }

    fn native_model_encode_body(&self) -> EncodeResult<Vec<u8>>
    where
        Self: Sized,
    {
        native_model_encode(self).map_err(|e| EncodeBodyError {
            msg: format!("{}", e),
            source: e.into(),
        })
    }

    fn native_model_decode_body(data: Vec<u8>, _id: u32) -> DecodeResult<Self>
    where
        Self: Sized,
    {
        native_model_decode(data).map_err(|e| DecodeBodyError::DecodeError {
            msg: format!("{}", e),
            source: e.into(),
        })
    }

    fn native_model_encode_downgrade_body(self, version: u32) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        println!(
            "B::serialization_and_downgrade({}, {})",
            version,
            Self::native_model_version()
        );
        if version == Self::native_model_version() {
            let result = self.native_model_encode_body()?;
            Ok(result)
        } else if version < Self::native_model_version() {
            A::native_model_encode_downgrade_body(self.into(), version)
        } else {
            panic!("Not implemented");
        }
    }
}

impl From<B> for A {
    fn from(_: B) -> Self {
        Self {}
    }
}

impl From<A> for B {
    fn from(_: A) -> Self {
        Self {}
    }
}

#[derive(Debug, Encode, Decode)]
struct C {}
impl Model for C {
    fn native_model_id() -> u32 {
        1
    }

    fn native_model_version() -> u32 {
        3
    }

    fn native_model_decode_upgrade_body(_data: Vec<u8>, id: u32, version: u32) -> Result<Self> {
        println!(
            "C::deserialization_and_upgrade({}, {})",
            version,
            Self::native_model_version()
        );
        if version == Self::native_model_version() {
            Ok(Self {})
        } else if version < Self::native_model_version() {
            let result = B::native_model_decode_upgrade_body(_data, id, version).map(|b| {
                b.try_into()
                    .map_err(|e: anyhow::Error| native_model::UpgradeError {
                        msg: format!("{}", e),
                        source: e.into(),
                    })
            })??;
            Ok(result)
        } else {
            panic!("Not implemented");
        }
    }

    fn native_model_encode_body(&self) -> EncodeResult<Vec<u8>>
    where
        Self: Sized,
    {
        native_model_encode(self).map_err(|e| EncodeBodyError {
            msg: format!("{}", e),
            source: e.into(),
        })
    }

    fn native_model_decode_body(data: Vec<u8>, _id: u32) -> DecodeResult<Self>
    where
        Self: Sized,
    {
        native_model_decode(data).map_err(|e| DecodeBodyError::DecodeError {
            msg: format!("{}", e),
            source: e.into(),
        })
    }

    fn native_model_encode_downgrade_body(self, version: u32) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        println!(
            "C::serialization_and_downgrade({}, {})",
            version,
            Self::native_model_version()
        );
        if version == Self::native_model_version() {
            let result = self.native_model_encode_body()?;
            Ok(result)
        } else if version < Self::native_model_version() {
            let result = B::native_model_encode_downgrade_body(
                self.try_into()
                    .map_err(|e: anyhow::Error| native_model::DowngradeError {
                        msg: format!("{}", e),
                        source: e.into(),
                    })?,
                version,
            )?;
            Ok(result)
        } else {
            panic!("Not implemented");
        }
    }
}

impl TryFrom<C> for B {
    type Error = anyhow::Error;

    fn try_from(_: C) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl TryFrom<B> for C {
    type Error = anyhow::Error;

    fn try_from(_: B) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

/**
I want to manage the upgrade and downgrade of native types using From and Into traits.
Let see 3 model A,B,C of a model id 1.
A is the oldest version of the model and is the version 1.
B is the intermediate version of the model and is the version 2.
C is the most recent version of the model and is the version 3.

We need to imagine that the data are serialized as a vector of bytes. The only things that we know
is the model id 1 and the version of the model.

I need to found an elegant way to deserialize the data as the most recent version of the model.
**/

#[test]
fn test_encode_downgrade() {
    let x = 3;
    let result = C::native_model_encode_downgrade_body(C {}, x);
    dbg!(&result);

    let x = 2;
    let result = C::native_model_encode_downgrade_body(C {}, x);
    dbg!(&result);

    let x = 1;
    let result = C::native_model_encode_downgrade_body(C {}, x);
    dbg!(&result);
}

#[test]
fn test_decode_upgrade() {
    let id = 1;
    let version = 3;
    let result = C::native_model_decode_upgrade_body(vec![], id, version);
    dbg!(&result);

    let version = 2;
    let result = C::native_model_decode_upgrade_body(vec![], id, version);
    dbg!(&result);

    let version = 1;
    let result = C::native_model_decode_upgrade_body(vec![], id, version);
    dbg!(&result);
}

fn native_model_decode_upgrade<T>(
    _data: Vec<u8>,
    model_id: u32,
    version: u32,
) -> native_model::Result<T>
where
    T: Model,
{
    if model_id == T::native_model_id() {
        T::native_model_decode_upgrade_body(_data, model_id, version)
    } else {
        panic!("The model id {} not supported", model_id);
    }
}

#[test]
fn test_decode_upgrade_c() {
    let x = 3;
    let result: C = native_model_decode_upgrade(vec![], 1, x).unwrap();
    dbg!(&result);

    let x = 2;
    let result: C = native_model_decode_upgrade(vec![], 1, x).unwrap();
    dbg!(&result);

    let x = 1;
    let result: C = native_model_decode_upgrade(vec![], 1, x).unwrap();
    dbg!(&result);
}

#[test]
fn test_decode_upgrade_b() {
    let x = 2;
    let result: B = native_model_decode_upgrade(vec![], 1, x).unwrap();
    dbg!(&result);

    // let x = 2;
    // let result: B = native_model_decode_upgrade(vec![], 1, x).unwrap();
    // dbg!(&result);
}
