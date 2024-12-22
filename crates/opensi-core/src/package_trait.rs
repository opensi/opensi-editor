use crate::node::{PackageNode, QuestionIdx, RoundIdx, ThemeIdx};

pub trait PackageBase: RoundContainer + Default + Clone {}
pub trait RoundBase: ThemesContainer + Default + Clone {}
pub trait ThemeBase: QuestionsContainer + Default + Clone {}
pub trait QuestionBase: Default + Clone {
    fn get_price(&self) -> usize;
    fn set_price(&mut self, price: usize);
}

pub trait RoundContainer {
    type Round: RoundBase;

    /// Get immutable reference to rounds.
    fn get_rounds(&self) -> &Vec<Self::Round>;
    /// Mutable reference to rounds.
    fn get_rounds_mut(&mut self) -> &mut Vec<Self::Round>;

    /// Return amount of [`Round`]s in this package.
    fn count_rounds(&self) -> usize {
        self.get_rounds().len()
    }

    /// Check if [`Round`] by index exist.
    fn contains_round(&self, idx: impl Into<RoundIdx>) -> bool {
        let idx = idx.into();
        *idx < self.count_rounds()
    }

    /// Get [`Round`] by index.
    fn get_round(&self, idx: impl Into<RoundIdx>) -> Option<&Self::Round> {
        let idx = idx.into();
        self.get_rounds().get(*idx)
    }

    /// Get mutable [`Round`] by index.
    fn get_round_mut(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Self::Round> {
        let idx = idx.into();
        self.get_rounds_mut().get_mut(*idx)
    }

    /// Remove [`Round`] by index and return it.
    fn remove_round(&mut self, idx: impl Into<RoundIdx>) -> Option<Self::Round> {
        let idx = idx.into();
        if *idx >= self.count_rounds() {
            return None;
        }
        self.get_rounds_mut().remove(*idx).into()
    }

    /// Push a new [`Round`] to the end of the package and
    /// return a reference to it.
    fn push_round(&mut self, round: Self::Round) -> &mut Self::Round {
        self.get_rounds_mut().push(round);
        self.get_rounds_mut().last_mut().unwrap()
    }

    /// Insert a new [`Round`] at position and return a
    /// reference to it.
    fn insert_round(
        &mut self,
        idx: impl Into<RoundIdx>,
        round: Self::Round,
    ) -> Option<&mut Self::Round> {
        let idx = idx.into();
        if *idx > self.count_rounds() {
            return None;
        }
        self.get_rounds_mut().insert(*idx, round);
        Some(&mut self.get_rounds_mut()[*idx])
    }

    /// Clone a [`Round`], push it afterwards and return
    /// a reference to the new round.
    fn duplicate_round(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Self::Round> {
        let idx = idx.into();
        self.get_round(idx).cloned().and_then(|round| self.insert_round(idx.next(), round))
    }

    /// Create a new default [`Round`], push it and return
    /// a reference to it.
    fn allocate_round(&mut self) -> &mut Self::Round {
        self.push_round(Self::Round::default())
    }
}

pub trait ThemesContainer {
    type Theme: ThemeBase;

    /// Get immutable reference to themes.
    fn get_themes(&self, idx: impl Into<RoundIdx>) -> Option<&Vec<Self::Theme>>;
    /// Mutable reference to themes.
    fn get_themes_mut(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Vec<Self::Theme>>;

    /// Return amount of [`Theme`]s in a [`Round`].
    fn count_themes(&self, idx: impl Into<RoundIdx>) -> usize {
        self.get_themes(idx).map(|themes| themes.len()).unwrap_or_default()
    }

    /// Check if [`Theme`] by indices exist.
    fn contains_theme(&self, idx: impl Into<ThemeIdx>) -> bool {
        let idx = idx.into();
        *idx < self.count_themes(idx.parent())
    }

    /// Get [`Theme`] in [`Round`] by indices.
    fn get_theme(&self, idx: impl Into<ThemeIdx>) -> Option<&Self::Theme> {
        let idx = idx.into();
        self.get_themes(idx.parent()).and_then(|themes| themes.get(*idx))
    }

    /// Get mutable [`Theme`] in [`Round`] by indices.
    fn get_theme_mut(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Self::Theme> {
        let idx = idx.into();
        self.get_themes_mut(idx.parent()).and_then(|themes| themes.get_mut(*idx))
    }

    /// Remove [`Theme`] in [`Round`] by indices.
    fn remove_theme(&mut self, idx: impl Into<ThemeIdx>) -> Option<Self::Theme> {
        let idx = idx.into();
        let themes = self.get_themes_mut(idx.parent())?;
        if *idx >= themes.len() {
            return None;
        }
        themes.remove(*idx).into()
    }

    /// Push a new [`Theme`] to the end of the [`Round`] and
    /// return a reference to it.
    fn push_theme(
        &mut self,
        idx: impl Into<RoundIdx>,
        theme: Self::Theme,
    ) -> Option<&mut Self::Theme> {
        let idx = idx.into();
        let themes = self.get_themes_mut(idx)?;
        themes.push(theme);
        themes.last_mut().unwrap().into()
    }

    /// Insert a new [`Theme`] at position and return a
    /// reference to it.
    fn insert_theme(
        &mut self,
        idx: impl Into<ThemeIdx>,
        theme: Self::Theme,
    ) -> Option<&mut Self::Theme> {
        let idx = idx.into();
        let themes = self.get_themes_mut(idx.parent())?;
        if *idx > themes.len() {
            return None;
        }
        themes.insert(*idx, theme);
        Some(&mut themes[*idx])
    }

    /// Clone a [`Theme`], push it afterwards and return
    /// a reference to the new theme.
    fn duplicate_theme(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Self::Theme> {
        let idx = idx.into();
        self.get_theme(idx).cloned().and_then(|theme| self.insert_theme(idx.next(), theme))
    }

    /// Create a new default [`Theme`], push it to the [`Round`]
    /// and return a reference to it.
    fn allocate_theme(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Self::Theme> {
        let idx = idx.into();
        self.push_theme(idx, Self::Theme::default())
    }
}

impl<R, C> ThemesContainer for C
where
    R: RoundBase + 'static,
    C: RoundContainer<Round = R>,
{
    type Theme = R::Theme;

    fn get_themes(&self, idx: impl Into<RoundIdx>) -> Option<&Vec<Self::Theme>> {
        let idx = idx.into();
        let rounds = self.get_round(idx)?;
        rounds.get_themes(idx)
    }

    fn get_themes_mut(&mut self, idx: impl Into<RoundIdx>) -> Option<&mut Vec<Self::Theme>> {
        let idx = idx.into();
        let rounds = self.get_round_mut(idx)?;
        rounds.get_themes_mut(idx)
    }
}

pub trait QuestionsContainer {
    type Question: QuestionBase;

    /// Get immutable reference to questions.
    fn get_questions(&self, idx: impl Into<ThemeIdx>) -> Option<&Vec<Self::Question>>;
    /// Mutable reference to questions.
    fn get_questions_mut(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Vec<Self::Question>>;
    /// Try to guess price for the next question. By defauit its:
    /// - Either a difference between the last two question prices;
    /// - Or the last question's price plus 100;
    ///
    /// In case of no questions, the default price is 100.
    fn guess_next_question_price(&self, idx: impl Into<ThemeIdx>) -> usize {
        let questions = self.get_questions(idx);
        let mut iter = questions.iter().copied().flatten().rev();
        match (iter.next(), iter.next()) {
            (Some(last), Some(prev)) => {
                let diff = last.get_price().abs_diff(prev.get_price());
                last.get_price() + diff
            },
            (Some(last), None) => last.get_price() + 100,
            _ => 100,
        }
    }

    /// Return amount of [`Question`]s in a [`Theme`].
    fn count_questions(&self, idx: impl Into<ThemeIdx>) -> usize {
        self.get_questions(idx).map(|questions| questions.len()).unwrap_or_default()
    }

    /// Check if [`Question`] by indices exist.
    fn contains_question(&self, idx: impl Into<QuestionIdx>) -> bool {
        let idx = idx.into();
        *idx < self.count_questions(idx.parent())
    }

    /// Get [`Question`] in [`Theme`] in [`Round`] by indices.
    fn get_question(&self, idx: impl Into<QuestionIdx>) -> Option<&Self::Question> {
        let idx = idx.into();
        self.get_questions(idx.parent()).and_then(|questions| questions.get(*idx))
    }

    /// Get mutable [`Question`] in [`Theme`] in [`Round`] by indices.
    fn get_question_mut(&mut self, idx: impl Into<QuestionIdx>) -> Option<&mut Self::Question> {
        let idx = idx.into();
        self.get_questions_mut(idx.parent()).and_then(|questions| questions.get_mut(*idx))
    }

    /// Remove [`Question`] in [`Theme`] in [`Round`] by indices.
    fn remove_question(&mut self, idx: impl Into<QuestionIdx>) -> Option<Self::Question> {
        let idx = idx.into();
        let questions = self.get_questions_mut(idx.parent())?;
        if *idx >= questions.len() {
            return None;
        }
        questions.remove(*idx).into()
    }

    /// Push a new [`Question`] to the end of the [`Theme`] in [`Round`]
    /// and return a reference to it.
    fn push_question(
        &mut self,
        idx: impl Into<ThemeIdx>,
        question: Self::Question,
    ) -> Option<&mut Self::Question> {
        let idx = idx.into();
        let questions = self.get_questions_mut(idx)?;
        questions.push(question);
        questions.last_mut().unwrap().into()
    }

    /// Insert a new [`Question`] at position and return a
    /// reference to it.
    fn insert_question(
        &mut self,
        idx: impl Into<QuestionIdx>,
        question: Self::Question,
    ) -> Option<&mut Self::Question> {
        let idx = idx.into();
        let questions = self.get_questions_mut(idx.parent())?;
        if *idx > questions.len() {
            return None;
        }
        questions.insert(*idx, question);
        Some(&mut questions[*idx])
    }

    /// Clone a [`Question`], push it afterwards and return
    /// a reference to the new question.
    fn duplicate_question(&mut self, idx: impl Into<QuestionIdx>) -> Option<&mut Self::Question> {
        let idx = idx.into();
        self.get_question(idx)
            .cloned()
            .and_then(|question| self.insert_question(idx.next(), question))
    }

    /// Create a new default [`Question`], push it to the [`Theme`] in [`Round`]
    /// and return a reference to it.
    fn allocate_question(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Self::Question> {
        let idx = idx.into();
        let mut question = Self::Question::default();
        question.set_price(self.guess_next_question_price(idx));
        self.push_question(idx, question)
    }
}

impl<T, C> QuestionsContainer for C
where
    T: ThemeBase + 'static,
    C: ThemesContainer<Theme = T>,
{
    type Question = T::Question;

    fn get_questions(&self, idx: impl Into<ThemeIdx>) -> Option<&Vec<Self::Question>> {
        let idx = idx.into();
        let theme = self.get_theme(idx)?;
        theme.get_questions(idx)
    }

    fn get_questions_mut(&mut self, idx: impl Into<ThemeIdx>) -> Option<&mut Vec<Self::Question>> {
        let idx = idx.into();
        let theme = self.get_theme_mut(idx)?;
        theme.get_questions_mut(idx)
    }
}

pub trait NodeContainer {
    fn duplicate_node(&mut self, node: PackageNode);
    fn allocate_node(&mut self, node: PackageNode);
    fn remove_node(&mut self, node: PackageNode);
}

impl<T> NodeContainer for T
where
    T: RoundContainer + 'static,
{
    fn duplicate_node(&mut self, node: PackageNode) {
        match node {
            PackageNode::Round(idx) => {
                self.duplicate_round(idx);
            },
            PackageNode::Theme(idx) => {
                self.duplicate_theme(idx);
            },
            PackageNode::Question(idx) => {
                self.duplicate_question(idx);
            },
        };
    }

    fn allocate_node(&mut self, node: PackageNode) {
        match node {
            PackageNode::Round(_) => {
                self.allocate_round();
            },
            PackageNode::Theme(idx) => {
                self.allocate_theme(idx.parent());
            },
            PackageNode::Question(idx) => {
                self.allocate_question(idx.parent());
            },
        };
    }

    fn remove_node(&mut self, node: PackageNode) {
        match node {
            PackageNode::Round(idx) => {
                self.remove_round(idx);
            },
            PackageNode::Theme(idx) => {
                self.remove_theme(idx);
            },
            PackageNode::Question(idx) => {
                self.remove_question(idx);
            },
        };
    }
}
