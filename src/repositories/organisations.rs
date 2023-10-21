use diesel::{PgConnection, QueryDsl, QueryResult};

use crate::models::organisation::{NewOrganisation, Organisation};
use crate::schema::organisations;

use diesel::prelude::*;

pub struct OrganisationRepository;

impl OrganisationRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Organisation> {
        organisations::table.find(id).get_result(c)
    }
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Organisation>> {
        organisations::table.limit(limit).load(c)
    }

    pub fn create(
        c: &mut PgConnection,
        new_organisation: NewOrganisation,
    ) -> QueryResult<Organisation> {
        diesel::insert_into(organisations::table)
            .values(new_organisation)
            .get_result(c)
    }

    pub fn update(
        c: &mut PgConnection,
        id: i32,
        organisation: Organisation,
    ) -> QueryResult<Organisation> {
        diesel::update(organisations::table.find(id))
            .set((organisations::name.eq(organisation.name),))
            .get_result(c)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(organisations::table.find(id)).execute(c)
    }
}
