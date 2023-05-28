use crate::enums::{ConvertBytesToHumanReadable,ErrorType};
use crate::structs::{LogEntry, LogEntryEncrypted, Task, TaskEncrypted};
use themis::secure_cell::SecureCell;
use themis::Error;

impl Task {
    pub fn encrypt_fields(task: &Task, key: Vec<u8>) -> Option<TaskEncrypted> {
        let mut vec_string_tasks: Vec<Vec<u8>> = vec![];
        let cell: Result<SecureCell, Error> = SecureCell::with_key(&key);

        let mut strings_task: [String; 4] =
            [String::new(), String::new(), String::new(), String::new()];
        match cell {
            Ok(cell) => {
                let sealed_cell = cell.seal();
                match &task.name {
                    Some(task_name) => {
                        strings_task[0] = task_name.to_string();
                    }
                    None => {
                        panic!("you must specify task name!");
                    }
                }
                match &task.shell {
                    Some(task_shell) => {
                        strings_task[1] = task_shell.to_string();
                    }
                    None => {
                        panic!("you must specify task shell!");
                    }
                }
                match &task.command {
                    Some(task_command) => {
                        strings_task[2] = task_command.to_string();
                    }
                    None => {
                        panic!("you must specify task command!");
                    }
                }
                match &task.comment {
                    Some(task_comment) => {
                        strings_task[3] = task_comment.to_string();
                    }
                    None => {
                        strings_task[3] = String::from("None");
                    }
                }
                for task in strings_task {
                    match sealed_cell.encrypt(task) {
                        Ok(enc_res) => {
                            vec_string_tasks.push(enc_res);
                        }
                        Err(_error) => {
                            ErrorType::error(&ErrorType::EncryptionError);
                        }
                    }
                }
                let option_ints_task = [
                    task.month,
                    task.day_of_month,
                    task.day_of_week,
                    task.hour,
                    task.minute,
                ];
                let ints_task = [
                    task.year_added,
                    task.month_added,
                    task.day_of_month_added,
                    task.hour_added,
                    task.minute_added,
                ];
                let mut tmp_holder_options: Vec<Option<Vec<u8>>> = vec![];
                let mut tmp_holder: Vec<Vec<u8>> = vec![];

                /*
                encrypt task user defined fields if and only if it contains value,
                if not push the none option into the stack.
                 */
                for option_ints in option_ints_task {
                    match option_ints {
                        Some(integar_value) => {
                            let res_enc = sealed_cell.encrypt(integar_value.to_le_bytes());
                            match res_enc {
                                Ok(enc_task) => {
                                    tmp_holder_options.push(Some(enc_task));
                                }
                                Err(_) => {
                                    tmp_holder_options.push(None);
                                }
                            }
                        }
                        None => {
                            tmp_holder_options.push(None);
                        }
                    }
                }
                for integar_task in ints_task {
                    match sealed_cell.encrypt(integar_task.to_le_bytes()) {
                        Ok(enc_res) => {
                            tmp_holder.push(enc_res);
                        }
                        Err(_error) => {
                            ErrorType::error(&ErrorType::EncryptionError);
                        }
                    }
                }

                let encrypted_struct = TaskEncrypted {
                    name: vec_string_tasks[0].clone(),
                    shell: vec_string_tasks[1].clone(),
                    command: vec_string_tasks[2].clone(),
                    comment: vec_string_tasks[3].clone(),
                    month: tmp_holder_options[0].clone(),
                    day_of_month: tmp_holder_options[1].clone(),
                    day_of_week: tmp_holder_options[2].clone(),
                    hour: tmp_holder_options[3].clone(),
                    minute: tmp_holder_options[4].clone(),
                    year_added: tmp_holder[0].clone(),
                    month_added: tmp_holder[1].clone(),
                    day_of_month_added: tmp_holder[2].clone(),
                    hour_added: tmp_holder[3].clone(),
                    minute_added: tmp_holder[4].clone(),
                };
                Some(encrypted_struct)
            }
            Err(_error) => None,
        }
    }
    pub fn decrypt_fields(tasks_encrypted: Vec<TaskEncrypted>, key: Vec<u8>) -> Option<Vec<Task>> {
        let mut tasks_table_decrypted: Vec<Task> = vec![];
        let cell: Result<SecureCell, Error> = SecureCell::with_key(&key);
        match cell {
            Ok(cell) => {
                let sealed_cell = cell.seal();
                for task in tasks_encrypted {
                    let name =
                        Task::convert_bytes_to_string(sealed_cell.decrypt(task.name).expect("
                        ERROR decrypting task name
                        "));
                    let shell =
                        Task::convert_bytes_to_string(sealed_cell.decrypt(task.shell).expect("
                        ERROR decrypting task shell
                        "));
                    let command =
                        Task::convert_bytes_to_string(sealed_cell.decrypt(task.command).expect("
                        ERROR decrypting task's command
                        "));
                    let comment =
                        Task::convert_bytes_to_string(sealed_cell.decrypt(task.comment).expect("
                        ERROR decrypting task comment
                        "));

                    let mut month: Option<i32> = None;
                    if let Some(task_month) = task.month {
                        month = Some(Task::convert_le_bytes_i32(
                            &sealed_cell.decrypt(task_month).expect(""),
                        ));
                    }
                    let mut day_of_month: Option<i32> = None;
                    if let Some(task_day_of_month) = task.day_of_month {
                        day_of_month = Some(Task::convert_le_bytes_i32(
                            &sealed_cell.decrypt(task_day_of_month).expect(""),
                        ));
                    }
                    let mut day_of_week: Option<i32> = None;
                    if let Some(task_day_of_week) = task.day_of_week {
                        day_of_week = Some(Task::convert_le_bytes_i32(
                            &sealed_cell.decrypt(task_day_of_week).expect(""),
                        ));
                    }
                    let mut hour: Option<i32> = None;
                    if let Some(task_hour) = task.hour {
                        hour = Some(Task::convert_le_bytes_i32(
                            &sealed_cell.decrypt(task_hour).expect(""),
                        ));
                    }
                    let mut minute: Option<i32> = None;
                    if let Some(task_minute) = task.minute {
                        minute = Some(Task::convert_le_bytes_i32(
                            &sealed_cell.decrypt(task_minute).expect(""),
                        ));
                    }

                    let year_added = Task::convert_le_bytes_i32(
                        &sealed_cell.decrypt(task.year_added.as_slice()).expect(""),
                    );

                    let month_added = Task::convert_le_bytes_i32(
                        &sealed_cell.decrypt(task.month_added.as_slice()).expect(""),
                    );
                    let day_of_month_added = Task::convert_le_bytes_i32(
                        &sealed_cell
                            .decrypt(task.day_of_month_added.as_slice())
                            .expect(""),
                    );
                    let hour_added = Task::convert_le_bytes_i32(
                        &sealed_cell.decrypt(task.hour_added.as_slice()).expect(""),
                    );
                    let minute_added = Task::convert_le_bytes_i32(
                        &sealed_cell.decrypt(task.minute_added.as_slice()).expect(""),
                    );
                    let task = Task {
                        name: Some(name),
                        shell: Some(shell),
                        command: Some(command),
                        comment: Some(comment),
                        month,
                        day_of_month,
                        day_of_week,
                        hour,
                        minute,
                        year_added,
                        month_added,
                        day_of_month_added,
                        hour_added,
                        minute_added,
                    };
                    tasks_table_decrypted.push(task);
                }
                Some(tasks_table_decrypted)
            }
            Err(_error) => {
                ErrorType::error(&ErrorType::DecryptionError);
                None
            }
        }
    }
}
impl LogEntry {
    pub fn encrypt_fields(&self, key: Vec<u8>) -> Option<LogEntryEncrypted> {
        let cell: Result<SecureCell, Error> = SecureCell::with_key(&key);
        match cell {
            Ok(cell_ok) => {
                let sealed_cell = cell_ok.seal();
                let mut output: Option<Vec<u8>> = None;
                if let Some(output_string) = &self.output {
                    let res_enc = sealed_cell.encrypt(output_string);
                    if let Ok(res_output_enc) = res_enc {
                        output = Some(res_output_enc);
                    }
                }
                let string_fields = [&self.name, &self.command];
                let int_fields = [
                    &self.execution_year,
                    &self.execution_month,
                    &self.execution_week,
                    &self.execution_day_of_month,
                    &self.execution_day_of_week,
                    &self.execution_hour,
                    &self.execution_minute,
                ];
                let mut tmp_holder: Vec<Vec<u8>> = vec![];

                for string in string_fields {
                    let res_enc = sealed_cell.encrypt(string);
                    match res_enc {
                        Ok(res_enc_ok) => tmp_holder.push(res_enc_ok),
                        Err(_) => {
                            dbg!("encryption error");
                        }
                    }
                }
                for int in int_fields {
                    let res_enc = sealed_cell.encrypt(int.to_le_bytes());
                    match res_enc {
                        Ok(res_enc_ok) => tmp_holder.push(res_enc_ok),
                        Err(_) => {
                            dbg!("encryption error");
                        }
                    }
                }
                let enc_log = LogEntryEncrypted {
                    name: tmp_holder[0].clone(),
                    command: tmp_holder[1].clone(),
                    output,
                    execution_year: tmp_holder[2].clone(),
                    execution_month: tmp_holder[3].clone(),
                    execution_week: tmp_holder[4].clone(),
                    execution_day_of_month: tmp_holder[5].clone(),
                    execution_day_of_week: tmp_holder[6].clone(),
                    execution_hour: tmp_holder[7].clone(),
                    execution_minute: tmp_holder[8].clone(),
                };
                Some(enc_log)
            }
            Err(_) => None,
        }
    }
    pub fn decrypt_fields(
        log_table: Vec<LogEntryEncrypted>,
        key: Vec<u8>,
    ) -> Option<Vec<LogEntry>> {
        let mut logs_table_decrypted: Vec<LogEntry> = vec![];
        let cell: Result<SecureCell, Error> = SecureCell::with_key(&key);
        match cell {
            Ok(cell_ok) => {
                let sealed_cell = cell_ok.seal();
                for log in log_table {
                    let name =
                        LogEntry::convert_bytes_to_string(sealed_cell.decrypt(log.name).expect(""));
                    let command = LogEntry::convert_bytes_to_string(
                        sealed_cell.decrypt(log.command).expect(""),
                    );
                    let mut output: Option<String> = None;
                    if let Some(output_some) = log.output {
                        output = Some(LogEntry::convert_bytes_to_string(
                            sealed_cell.decrypt(output_some).expect(""),
                        ));
                    }
                    let execution_year = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_year).expect(""),
                    );

                    let execution_month = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_month).expect(""),
                    );
                    let execution_week = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_week).expect(""),
                    );

                    let execution_day_of_month = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_day_of_month).expect(""),
                    );
                    let execution_day_of_week = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_day_of_week).expect(""),
                    );
                    let execution_hour = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_hour).expect(""),
                    );
                    let execution_minute = LogEntry::convert_le_bytes_i32(
                        &sealed_cell.decrypt(log.execution_minute).expect(""),
                    );

                    let log_entry = LogEntry {
                        name,
                        command,
                        output,
                        execution_year,
                        execution_month,
                        execution_week,
                        execution_day_of_month,
                        execution_day_of_week,
                        execution_hour,
                        execution_minute,
                    };
                    logs_table_decrypted.push(log_entry);
                }
                Some(logs_table_decrypted)
            }
            Err(_error) => None,
        }
    }
}
