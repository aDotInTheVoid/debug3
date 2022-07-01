use crate::Debug;

impl Debug for aws_sdk_ec2::model::RiProductDescription {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_tuple("RiProductDescription")
            .field(&self.as_str())
            .finish()
    }
}
