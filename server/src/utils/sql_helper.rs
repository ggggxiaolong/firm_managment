pub struct SqlHelper {
    pub sql: String,
    has_where: bool,
}

impl SqlHelper {
    pub fn query(table: &str, columns: &str) -> Self {
        let mut helper = SqlHelper {
            sql: String::new(),
            has_where: false,
        };
        helper.sql.push_str("SELECT ");
        helper.sql.push_str(columns);
        helper.sql.push_str(" FROM ");
        helper.sql.push_str(table);
        helper
    }

    pub fn update(table: &str, columns: &str) -> Self {
        let mut helper = SqlHelper {
            sql: String::new(),
            has_where: false,
        };
        helper.sql.push_str("UPDATE ");
        helper.sql.push_str(table);
        helper.sql.push_str(" SET ");
        for colum in columns.split(',').collect::<Vec<&str>>(){
            helper.sql.push_str(colum);
            helper.sql.push_str(" = ?,");
        }
        helper.sql.pop();
        helper.sql.push(' ');
        helper
    }

    pub fn insert(table: &str, columns: &str) -> Self {
        let mut helper = SqlHelper {
            sql: String::new(),
            has_where: false,
        };
        helper.sql.push_str("INSERT INTO ");
        helper.sql.push_str(table);
        helper.sql.push_str(" ( ");
        helper.sql.push_str(columns);
        helper.sql.push_str(" ) ");
        helper.sql.push_str("VALUES (");
        for _ in columns.split(',').collect::<Vec<&str>>(){
            helper.sql.push_str(" ?,");
        }
        helper.sql.pop();
        helper.sql.push_str(" ) ");
        helper
    }

    pub fn delete(table: &str) -> Self {
        let mut helper = SqlHelper {
            sql: String::new(),
            has_where: false,
        };
        helper.sql.push_str("DELETE FROM ");
        helper.sql.push_str(table);
        helper.sql.push(' ');
        helper
    }

    pub fn and_where_eq(&mut self, column_name: &str) -> &mut Self {
        if self.has_where {
            self.sql.push_str(" AND ");
        } else {
            self.has_where = true
        }
        self.sql.push_str(" WHERE ");
        self.sql.push_str(column_name);
        self.sql.push_str(" = ?");
        self
    }

    pub fn and_where_not_eq(&mut self, column_name: &str) -> &mut Self {
        if self.has_where {
            self.sql.push_str(" AND ");
        } else {
            self.has_where = true
        }
        self.sql.push_str(" WHERE ");
        self.sql.push_str(column_name);
        self.sql.push_str(" <> ?");
        self
    }

    pub fn and_where_like(&mut self, column_name: &str) -> &mut Self {
        if self.has_where {
            self.sql.push_str(" AND ");
        } else {
            self.has_where = true
        }
        self.sql.push_str(" WHERE ");
        self.sql.push_str(column_name);
        self.sql.push_str(" like %?%");
        self
    }

    pub fn page(&mut self, page: i32, page_size: i32) -> &mut Self {
        self.sql.push_str(" LIMIT ");
        self.sql.push_str(&page_size.to_string());
        self.sql.push_str(" OFFSET ");
        self.sql.push_str(&(page_size * page).to_string());
        self
    }

    pub fn order_desc(&mut self, column_name: &str)-> &mut Self {
        self.sql.push_str(" ORDER BY ");
        self.sql.push_str(column_name);
        self.sql.push_str(" DESC ");
        self
    }

    pub fn order_asc(&mut self, column_name: &str)-> &mut Self {
        self.sql.push_str(" ORDER BY ");
        self.sql.push_str(column_name);
        self.sql.push_str(" ASC");
        self
    }

    pub fn sql(&self) -> String {
        self.sql.clone()
    }
}

#[cfg(test)]
mod test {
    use super::SqlHelper;


    #[test]
    pub fn test_intser(){
        let sql = SqlHelper::insert("table", "columns").sql();
        assert_eq!(&sql, "INSERT INTO table [( columns )] VALUES ( ? ) ");
        let sql = SqlHelper::insert("table", "id, name").sql();
        assert_eq!(&sql, "INSERT INTO table [( id, name )] VALUES ( ?, ? ) ");
    }

    #[test]
    pub fn test_update(){
        let sql = SqlHelper::update("table", "columns").sql();
        assert_eq!(&sql, "UPDATE table SET columns = ? ");
        let sql = SqlHelper::update("table", "id, name").sql();
        assert_eq!(&sql, "UPDATE table SET id = ?, name = ? ");
    }
}
